//! Random inputs for stress testing.
//!
//! [`Rng`] is a deterministic pseudorandom generator, and the functions build the values, sequences
//! and graphs that appear in competitive programming inputs. Every function takes `rng` first, so
//! that one seed determines the whole input.

/// A pseudorandom generator (SplitMix64).
///
/// # Definition
/// `Rng::new(seed)` and the sequence of values it produces are determined by `seed`.
pub struct Rng {
    state: u64,
}

impl Rng {
    /// The generator with the seed `seed`.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// The next value in `[0, 2^64)`.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// The next value in `[0, n)`.
    /// # Panics
    /// Panics if `n = 0`.
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0, "n must be positive");
        self.next_u64() % n
    }

    /// The next value in `[l, r)`.
    /// # Panics
    /// Panics if `l >= r`.
    pub fn range(&mut self, l: i64, r: i64) -> i64 {
        assert!(l < r, "l must be less than r: l={l}, r={r}");
        l + self.below(r.abs_diff(l)) as i64
    }

    /// The next boolean.
    pub fn bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Permutes `a` uniformly at random.
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in (1..a.len()).rev() {
            a.swap(i, self.below(i as u64 + 1) as usize);
        }
    }
}

/// A permutation of `[0, n)`.
pub fn permutation(rng: &mut Rng, n: usize) -> Vec<usize> {
    let mut a: Vec<usize> = (0..n).collect();
    rng.shuffle(&mut a);
    a
}

/// `n` values in `[l, r)`.
/// # Panics
/// Panics if `l >= r`.
pub fn values(rng: &mut Rng, n: usize, l: i64, r: i64) -> Vec<i64> {
    (0..n).map(|_| rng.range(l, r)).collect()
}

/// `n` distinct values in `[l, r)`.
/// # Panics
/// Panics if `l >= r`, or `n > r - l`.
pub fn distinct_values(rng: &mut Rng, n: usize, l: i64, r: i64) -> Vec<i64> {
    assert!(l < r, "l must be less than r: l={l}, r={r}");
    let width = r.abs_diff(l);
    assert!(
        n as u128 <= width as u128,
        "n must be at most r - l: n={n}, l={l}, r={r}"
    );
    if 2 * n as u128 > width as u128 {
        let mut a: Vec<i64> = (0..width as usize).map(|i| l + i as i64).collect();
        rng.shuffle(&mut a);
        a.truncate(n);
        return a;
    }
    let mut seen = std::collections::HashSet::new();
    while seen.len() < n {
        seen.insert(rng.range(l, r));
    }
    seen.into_iter().collect()
}

/// A string of `n` characters of `alphabet`.
/// # Panics
/// Panics if `alphabet` is empty.
pub fn string(rng: &mut Rng, n: usize, alphabet: &[u8]) -> String {
    assert!(!alphabet.is_empty(), "alphabet must not be empty");
    (0..n)
        .map(|_| alphabet[rng.below(alphabet.len() as u64) as usize] as char)
        .collect()
}

/// A string of `n` lowercase letters.
pub fn lowercase(rng: &mut Rng, n: usize) -> String {
    string(rng, n, b"abcdefghijklmnopqrstuvwxyz")
}

/// A string of `n` uppercase letters.
pub fn uppercase(rng: &mut Rng, n: usize) -> String {
    string(rng, n, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ")
}

/// A string of `n` digits.
pub fn digits(rng: &mut Rng, n: usize) -> String {
    string(rng, n, b"0123456789")
}

/// The `h` rows of an `h x w` grid of characters of `alphabet`.
/// # Panics
/// Panics if `alphabet` is empty.
pub fn grid(rng: &mut Rng, h: usize, w: usize, alphabet: &[u8]) -> Vec<String> {
    (0..h).map(|_| string(rng, w, alphabet)).collect()
}

/// The `n - 1` edges of a tree on `[0, n)`, in random order with random labels.
/// # Panics
/// Panics if `n = 0`.
pub fn tree(rng: &mut Rng, n: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    let edges = (1..n).map(|i| (rng.below(i as u64) as usize, i)).collect();
    relabel_undirected(rng, n, edges)
}

/// The `m` edges of an undirected graph on `[0, n)`, with loops and multiple edges, in random order
/// with random labels.
/// # Panics
/// Panics if `n = 0`.
pub fn graph(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    let edges = (0..m)
        .map(|_| (rng.below(n as u64) as usize, rng.below(n as u64) as usize))
        .collect();
    relabel_undirected(rng, n, edges)
}

/// The `m` edges of a directed graph on `[0, n)`, with loops and multiple edges, in random order
/// with random labels.
/// # Panics
/// Panics if `n = 0`.
pub fn digraph(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    let edges = (0..m)
        .map(|_| (rng.below(n as u64) as usize, rng.below(n as u64) as usize))
        .collect();
    relabel(rng, n, edges)
}

/// The `m` edges of a simple undirected graph on `[0, n)`, in random order with random labels.
/// # Panics
/// Panics if `n = 0`, or `m > n (n - 1) / 2`.
pub fn simple_graph(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    assert!(
        m <= n * (n - 1) / 2,
        "m must be at most n (n - 1) / 2: n={n}, m={m}"
    );
    let mut seen = std::collections::HashSet::new();
    while seen.len() < m {
        let (u, v) = (rng.below(n as u64) as usize, rng.below(n as u64) as usize);
        if u != v {
            seen.insert((u.min(v), u.max(v)));
        }
    }
    relabel_undirected(rng, n, seen.into_iter().collect())
}

/// The `m` edges of a simple directed graph on `[0, n)`, in random order with random labels.
/// # Panics
/// Panics if `n = 0`, or `m > n (n - 1)`.
pub fn simple_digraph(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    assert!(m <= n * (n - 1), "m must be at most n (n - 1): n={n}, m={m}");
    let mut seen = std::collections::HashSet::new();
    while seen.len() < m {
        let (u, v) = (rng.below(n as u64) as usize, rng.below(n as u64) as usize);
        if u != v {
            seen.insert((u, v));
        }
    }
    relabel(rng, n, seen.into_iter().collect())
}

/// The `m` edges of a connected simple undirected graph on `[0, n)`, in random order with random
/// labels.
/// # Panics
/// Panics if `n = 0`, `m < n - 1`, or `m > n (n - 1) / 2`.
pub fn connected_graph(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    assert!(m >= n - 1, "m must be at least n - 1: n={n}, m={m}");
    assert!(
        m <= n * (n - 1) / 2,
        "m must be at most n (n - 1) / 2: n={n}, m={m}"
    );
    let mut seen: std::collections::HashSet<(usize, usize)> = (1..n)
        .map(|i| (rng.below(i as u64) as usize, i))
        .collect();
    while seen.len() < m {
        let (u, v) = (rng.below(n as u64) as usize, rng.below(n as u64) as usize);
        if u != v {
            seen.insert((u.min(v), u.max(v)));
        }
    }
    relabel_undirected(rng, n, seen.into_iter().collect())
}

/// The `n - 1` edges of a path on `[0, n)`, in random order with random labels.
/// # Panics
/// Panics if `n = 0`.
pub fn path(rng: &mut Rng, n: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    let edges = (1..n).map(|i| (i - 1, i)).collect();
    relabel_undirected(rng, n, edges)
}

/// The `n - 1` edges of a star on `[0, n)`, in random order with random labels.
/// # Panics
/// Panics if `n = 0`.
pub fn star(rng: &mut Rng, n: usize) -> Vec<(usize, usize)> {
    assert!(n > 0, "n must be positive");
    let edges = (1..n).map(|i| (0, i)).collect();
    relabel_undirected(rng, n, edges)
}

/// The edges with the vertices relabeled by a random permutation of `[0, n)`, and the edges
/// shuffled.
fn relabel(rng: &mut Rng, n: usize, mut edges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let label = permutation(rng, n);
    for (u, v) in edges.iter_mut() {
        *u = label[*u];
        *v = label[*v];
    }
    rng.shuffle(&mut edges);
    edges
}

/// The edges as by [`relabel`], with the two endpoints of each edge in random order.
fn relabel_undirected(rng: &mut Rng, n: usize, edges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut edges = relabel(rng, n, edges);
    for (u, v) in edges.iter_mut() {
        if rng.bool() {
            std::mem::swap(u, v);
        }
    }
    edges
}
