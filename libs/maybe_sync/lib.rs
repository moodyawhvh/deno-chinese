// Copyright 2018-2026 the Deno authors. MIT license.

// 🌐 中文注释(汉化仓库添加):本 crate 提供"maybe sync"抽象层。
// Deno 同时面向两类场景构建:多线程(需要 `Send`/`Sync`/`Arc`)与
// 单线程(只能使用 `Rc`/`RefCell` 等非线程安全类型)。
// 通过 `sync` feature 在编译期切换两套实现,让上层代码用同一组
// `Maybe*` 类型别名书写,无需关心目标配置是否启用线程。

pub use inner::*;

// 启用 `sync` feature 时:直接映射到线程安全的真实类型。
#[cfg(feature = "sync")]
mod inner {
  #![allow(
    clippy::disallowed_types,
    reason = "implementation the rule says to use"
  )]

  // MaybeSend/MaybeSync = 真正的 Send/Sync 标记 trait
  pub use core::marker::Send as MaybeSend;
  pub use core::marker::Sync as MaybeSync;
  // MaybeArc = 线程安全引用计数 Arc
  pub use std::sync::Arc as MaybeArc;
  pub use std::sync::OnceLock as MaybeOnceLock;

  // 并发哈希表/集合,默认使用 FxHash 构建器(性能优先)
  pub type MaybeDashMap<K, V, S = rustc_hash::FxBuildHasher> =
    dashmap::DashMap<K, V, S>;
  pub type MaybeDashSet<T, S = rustc_hash::FxBuildHasher> =
    dashmap::DashSet<T, S>;
}

// 未启用 `sync` feature(单线程模式):用 Rc/RefCell 等替身。
#[cfg(not(feature = "sync"))]
mod inner {
  pub use std::cell::OnceCell as MaybeOnceLock;
  use std::cell::Ref;
  use std::cell::RefCell;
  use std::collections::HashMap;
  use std::hash::BuildHasher;
  use std::hash::Hash;
  // 单线程下用 Rc 代替 Arc(无原子操作开销)
  pub use std::rc::Rc as MaybeArc;

  use rustc_hash::FxBuildHasher;

  // 空实现:单线程模式下所有类型都"无需" Send/Sync 约束,
  // 因此这两个 trait 对任意类型都自动满足(包括 ?Sized)。
  pub trait MaybeSync {}
  impl<T> MaybeSync for T where T: ?Sized {}
  pub trait MaybeSend {}
  impl<T> MaybeSend for T where T: ?Sized {}

  // 包装结构体,只暴露 `DashMap` API 的一个子集。
  // 内部用 RefCell<HashMap> 模拟并发 map 的接口语义。
  pub struct MaybeDashMap<K, V, S = FxBuildHasher>(RefCell<HashMap<K, V, S>>);

  // 手动实现 Debug,避免要求 `S: Debug`
  // (例如 `FxBuildHasher` 没有实现 `Debug`)
  impl<K: std::fmt::Debug, V: std::fmt::Debug, S> std::fmt::Debug
    for MaybeDashMap<K, V, S>
  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      std::fmt::Debug::fmt(&self.0, f)
    }
  }

  impl<K, V, S> Default for MaybeDashMap<K, V, S>
  where
    K: Eq + Hash,
    S: Default + BuildHasher + Clone,
  {
    fn default() -> Self {
      Self(RefCell::new(Default::default()))
    }
  }

  impl<K: Eq + Hash, V, S: BuildHasher> MaybeDashMap<K, V, S> {
    // 读取:返回 Ref 引用,借用期间不允许写入(与 DashMap 分片锁语义对齐)
    pub fn get<'a, Q: Eq + Hash + ?Sized>(
      &'a self,
      key: &Q,
    ) -> Option<Ref<'a, V>>
    where
      K: std::borrow::Borrow<Q>,
    {
      Ref::filter_map(self.0.borrow(), |map| map.get(key)).ok()
    }

    // 插入:返回被替换的旧值(若有)
    pub fn insert(&self, key: K, value: V) -> Option<V> {
      let mut inner = self.0.borrow_mut();
      inner.insert(key, value)
    }

    pub fn clear(&self) {
      self.0.borrow_mut().clear();
    }

    // 删除:返回 (key, value) 以匹配 DashMap::remove 的签名
    pub fn remove(&self, key: &K) -> Option<(K, V)> {
      self.0.borrow_mut().remove_entry(key)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
      self.0.borrow().len()
    }
  }

  // 包装结构体,只暴露 `DashSet` API 的一个子集。
  pub struct MaybeDashSet<V, S = FxBuildHasher>(
    RefCell<std::collections::HashSet<V, S>>,
  );

  // 手动实现 Debug,避免要求 `S: Debug`
  // (例如 `FxBuildHasher` 没有实现 `Debug`)
  impl<V: std::fmt::Debug, S> std::fmt::Debug for MaybeDashSet<V, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      std::fmt::Debug::fmt(&self.0, f)
    }
  }

  impl<V, S> Default for MaybeDashSet<V, S>
  where
    V: Eq + Hash,
    S: Default + BuildHasher + Clone,
  {
    fn default() -> Self {
      Self(RefCell::new(Default::default()))
    }
  }

  impl<V: Eq + Hash, S: BuildHasher> MaybeDashSet<V, S> {
    // 插入:返回是否为新插入(false 表示值已存在)
    pub fn insert(&self, value: V) -> bool {
      let mut inner = self.0.borrow_mut();
      inner.insert(value)
    }
  }
}

// 构造 MaybeArc(单线程=Rc,多线程=Arc),供上层"按配置选择智能指针"。
#[allow(
  clippy::disallowed_types,
  reason = "implementation the rule says to use"
)]
#[inline]
pub fn new_rc<T>(value: T) -> MaybeArc<T> {
  MaybeArc::new(value)
}

// 永远构造真正的 Arc(无论是否启用 sync feature),
// 适用于明确需要线程安全共享的场景。
#[allow(
  clippy::disallowed_types,
  reason = "implementation the rule says to use"
)]
#[inline]
pub fn new_arc<T>(value: T) -> std::sync::Arc<T> {
  std::sync::Arc::new(value)
}
