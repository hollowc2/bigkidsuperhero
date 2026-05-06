//! Level data structures and registry — all 7 level definitions.
//!
//! The `level_layout!` macro is defined in this module so internal helper macros
//! (`count!`, `__platforms!`, etc.) remain in scope when the macro is invoked.

use bevy::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers (defined first so subsequent macros can reference them)
// ─────────────────────────────────────────────────────────────────────────────

/// Count comma-separated items (0–19).
macro_rules! count {
    () => { 0 };
    ($a:expr) => { 1 };
    ($a:expr, $b:expr) => { 2 };
    ($a:expr, $b:expr, $c:expr) => { 3 };
    ($a:expr, $b:expr, $c:expr, $d:expr) => { 4 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => { 5 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => { 6 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => { 7 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr) => { 8 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr) => { 9 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr) => { 10 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr) => { 11 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr) => { 12 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr) => { 13 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr) => { 14 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr, $o:expr) => { 15 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr, $o:expr, $p:expr) => { 16 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr, $o:expr, $p:expr, $q:expr) => { 17 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr, $o:expr, $p:expr, $q:expr, $r:expr) => { 18 };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr, $o:expr, $p:expr, $q:expr, $r:expr, $s:expr) => { 19 };
}

/// Zero-fill value for padding.
macro_rules! z {
    (PlatformData) => { PlatformData { x: 0.0, y: 0.0, width: 0.0, tint: Color::NONE, moving: None } };
    (HazardData)   => { HazardData   { x: 0.0, y: 0.0, kind: HazardKind::Spike } };
    (MonsterData)  => { MonsterData  { x: 0.0, y: 0.0, start_x: 0.0, end_x: 0.0, speed: 0.0 } };
    ((f32,f32))    => { (0.0_f32, 0.0_f32) };
}

macro_rules! __assign {
    ($a:ident, $i:expr, $v:expr) => { $a[$i] = $v; };
}

macro_rules! __platforms {
    ($a:ident, []) => {};
    ($a:ident, [ $p0:expr $(,)? ]) => { __assign!($a, 0, $p0); };
    ($a:ident, [ $p0:expr, $p1:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr, $p12:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); __assign!($a, 12, $p12); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr, $p12:expr, $p13:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); __assign!($a, 12, $p12); __assign!($a, 13, $p13); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr, $p12:expr, $p13:expr, $p14:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); __assign!($a, 12, $p12); __assign!($a, 13, $p13); __assign!($a, 14, $p14); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr, $p12:expr, $p13:expr, $p14:expr, $p15:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); __assign!($a, 12, $p12); __assign!($a, 13, $p13); __assign!($a, 14, $p14); __assign!($a, 15, $p15); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr, $p12:expr, $p13:expr, $p14:expr, $p15:expr, $p16:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); __assign!($a, 12, $p12); __assign!($a, 13, $p13); __assign!($a, 14, $p14); __assign!($a, 15, $p15); __assign!($a, 16, $p16); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr, $p12:expr, $p13:expr, $p14:expr, $p15:expr, $p16:expr, $p17:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); __assign!($a, 12, $p12); __assign!($a, 13, $p13); __assign!($a, 14, $p14); __assign!($a, 15, $p15); __assign!($a, 16, $p16); __assign!($a, 17, $p17); };
    ($a:ident, [ $p0:expr, $p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr, $p10:expr, $p11:expr, $p12:expr, $p13:expr, $p14:expr, $p15:expr, $p16:expr, $p17:expr, $p18:expr $(,)? ]) => { __assign!($a, 0, $p0); __assign!($a, 1, $p1); __assign!($a, 2, $p2); __assign!($a, 3, $p3); __assign!($a, 4, $p4); __assign!($a, 5, $p5); __assign!($a, 6, $p6); __assign!($a, 7, $p7); __assign!($a, 8, $p8); __assign!($a, 9, $p9); __assign!($a, 10, $p10); __assign!($a, 11, $p11); __assign!($a, 12, $p12); __assign!($a, 13, $p13); __assign!($a, 14, $p14); __assign!($a, 15, $p15); __assign!($a, 16, $p16); __assign!($a, 17, $p17); __assign!($a, 18, $p18); };
}

macro_rules! __collectibles {
    ($a:ident, []) => {};
    ($a:ident, [ $c0:expr $(,)? ]) => { __assign!($a, 0, $c0); };
    ($a:ident, [ $c0:expr, $c1:expr $(,)? ]) => { __assign!($a, 0, $c0); __assign!($a, 1, $c1); };
    ($a:ident, [ $c0:expr, $c1:expr, $c2:expr $(,)? ]) => { __assign!($a, 0, $c0); __assign!($a, 1, $c1); __assign!($a, 2, $c2); };
    ($a:ident, [ $c0:expr, $c1:expr, $c2:expr, $c3:expr $(,)? ]) => { __assign!($a, 0, $c0); __assign!($a, 1, $c1); __assign!($a, 2, $c2); __assign!($a, 3, $c3); };
    ($a:ident, [ $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr $(,)? ]) => { __assign!($a, 0, $c0); __assign!($a, 1, $c1); __assign!($a, 2, $c2); __assign!($a, 3, $c3); __assign!($a, 4, $c4); };
    ($a:ident, [ $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr $(,)? ]) => { __assign!($a, 0, $c0); __assign!($a, 1, $c1); __assign!($a, 2, $c2); __assign!($a, 3, $c3); __assign!($a, 4, $c4); __assign!($a, 5, $c5); };
    ($a:ident, [ $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr $(,)? ]) => { __assign!($a, 0, $c0); __assign!($a, 1, $c1); __assign!($a, 2, $c2); __assign!($a, 3, $c3); __assign!($a, 4, $c4); __assign!($a, 5, $c5); __assign!($a, 6, $c6); };
    ($a:ident, [ $c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr, $c5:expr, $c6:expr, $c7:expr $(,)? ]) => { __assign!($a, 0, $c0); __assign!($a, 1, $c1); __assign!($a, 2, $c2); __assign!($a, 3, $c3); __assign!($a, 4, $c4); __assign!($a, 5, $c5); __assign!($a, 6, $c6); __assign!($a, 7, $c7); };
}

macro_rules! __hazards {
    ($a:ident, []) => {};
    ($a:ident, [ $h0:expr $(,)? ]) => { __assign!($a, 0, $h0); };
    ($a:ident, [ $h0:expr, $h1:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr, $h12:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); __assign!($a, 12, $h12); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr, $h12:expr, $h13:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); __assign!($a, 12, $h12); __assign!($a, 13, $h13); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr, $h12:expr, $h13:expr, $h14:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); __assign!($a, 12, $h12); __assign!($a, 13, $h13); __assign!($a, 14, $h14); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr, $h12:expr, $h13:expr, $h14:expr, $h15:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); __assign!($a, 12, $h12); __assign!($a, 13, $h13); __assign!($a, 14, $h14); __assign!($a, 15, $h15); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr, $h12:expr, $h13:expr, $h14:expr, $h15:expr, $h16:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); __assign!($a, 12, $h12); __assign!($a, 13, $h13); __assign!($a, 14, $h14); __assign!($a, 15, $h15); __assign!($a, 16, $h16); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr, $h12:expr, $h13:expr, $h14:expr, $h15:expr, $h16:expr, $h17:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); __assign!($a, 12, $h12); __assign!($a, 13, $h13); __assign!($a, 14, $h14); __assign!($a, 15, $h15); __assign!($a, 16, $h16); __assign!($a, 17, $h17); };
    ($a:ident, [ $h0:expr, $h1:expr, $h2:expr, $h3:expr, $h4:expr, $h5:expr, $h6:expr, $h7:expr, $h8:expr, $h9:expr, $h10:expr, $h11:expr, $h12:expr, $h13:expr, $h14:expr, $h15:expr, $h16:expr, $h17:expr, $h18:expr $(,)? ]) => { __assign!($a, 0, $h0); __assign!($a, 1, $h1); __assign!($a, 2, $h2); __assign!($a, 3, $h3); __assign!($a, 4, $h4); __assign!($a, 5, $h5); __assign!($a, 6, $h6); __assign!($a, 7, $h7); __assign!($a, 8, $h8); __assign!($a, 9, $h9); __assign!($a, 10, $h10); __assign!($a, 11, $h11); __assign!($a, 12, $h12); __assign!($a, 13, $h13); __assign!($a, 14, $h14); __assign!($a, 15, $h15); __assign!($a, 16, $h16); __assign!($a, 17, $h17); __assign!($a, 18, $h18); };
}

macro_rules! __monsters {
    ($a:ident, []) => {};
    ($a:ident, [ $m0:expr $(,)? ]) => { __assign!($a, 0, $m0); };
    ($a:ident, [ $m0:expr, $m1:expr $(,)? ]) => { __assign!($a, 0, $m0); __assign!($a, 1, $m1); };
    ($a:ident, [ $m0:expr, $m1:expr, $m2:expr $(,)? ]) => { __assign!($a, 0, $m0); __assign!($a, 1, $m1); __assign!($a, 2, $m2); };
}

// ── level_layout! macro ──────────────────────────────────────────────────────

/// Construct a LevelLayout in const context.
#[macro_export]
macro_rules! level_layout {
    (
        ground: $gx:expr, $gy:expr, $gw:expr;
        platforms: [ $($p:expr),* $(,)? ];
        collectibles: [ $($c:expr),* $(,)? ];
        hazards: [ $($h:expr),* $(,)? ];
        monsters: [ $($m:expr),* $(,)? ];
        flag: $fx:expr, $fy:expr;
        bound: $bound:expr;
        theme: $theme:expr
    ) => {{
        const PN: usize = count!($($p),*);
        const CN: usize = count!($($c),*);
        const HN: usize = count!($($h),*);
        const MN: usize = count!($($m),*);

        const PLATFORMS: [PlatformData; MAX_PLATFORMS] = {
            #[allow(unused_mut)]
            let mut a = [z!(PlatformData); MAX_PLATFORMS];
            __platforms!(a, [ $($p),* ]);
            a
        };
        const COLLECTIBLES: [(f32, f32); MAX_COLLECTIBLES] = {
            #[allow(unused_mut)]
            let mut b = [z!((f32,f32)); MAX_COLLECTIBLES];
            __collectibles!(b, [ $($c),* ]);
            b
        };
        const HAZARDS: [HazardData; MAX_HAZARDS] = {
            #[allow(unused_mut)]
            let mut c = [z!(HazardData); MAX_HAZARDS];
            __hazards!(c, [ $($h),* ]);
            c
        };
        const MONSTERS: [MonsterData; MAX_MONSTERS] = {
            #[allow(unused_mut)]
            let mut d = [z!(MonsterData); MAX_MONSTERS];
            __monsters!(d, [ $($m),* ]);
            d
        };

        LevelLayout {
            ground: ($gx, $gy, $gw),
            platforms:    PLATFORMS,
            platform_n:    PN,
            collectibles:  COLLECTIBLES,
            collectible_n: CN,
            hazards:       HAZARDS,
            hazard_n:       HN,
            monsters:      MONSTERS,
            monster_n:     MN,
            flag: ($fx, $fy),
            right_bound: $bound,
            theme: $theme,
        }
    }};
}

// ─────────────────────────────────────────────────────────────────────────────
// Data structures
// ─────────────────────────────────────────────────────────────────────────────

// Allow dead code for data structures defined for future use
#[allow(dead_code)]

/// Theme/color palette for each level.
#[derive(Clone, Copy, PartialEq)]
pub enum LevelTheme {
    Grassland,
    RockyMountain,
    WaterWorld,
    FieryFurnace,
    FrigidFreezer,
    ElectricStar,
    Boss,
}

impl LevelTheme {
    /// Background clear color for this theme.
    pub fn clear_color(&self) -> Color {
        match self {
            LevelTheme::Grassland     => Color::srgb(0.4, 0.7, 1.0),
            LevelTheme::RockyMountain => Color::srgb(0.55, 0.82, 1.0),
            LevelTheme::WaterWorld   => Color::srgb(0.15, 0.35, 0.65),
            LevelTheme::FieryFurnace => Color::srgb(0.22, 0.06, 0.04),
            LevelTheme::FrigidFreezer => Color::srgb(0.6, 0.85, 1.0),
            LevelTheme::ElectricStar => Color::srgb(0.08, 0.04, 0.18),
            LevelTheme::Boss         => Color::srgb(0.05, 0.0, 0.08),
        }
    }
}

/// Max counts per level (set to the largest level).
pub const MAX_PLATFORMS:    usize = 19;
pub const MAX_COLLECTIBLES: usize = 8;
pub const MAX_HAZARDS:      usize = 19;
pub const MAX_MONSTERS:     usize = 3;

/// A static or moving platform definition.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct PlatformData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub tint: Color,
    pub moving: Option<MovingData>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct MovingData {
    pub amplitude: f32,
    pub speed: f32,
    pub horizontal: bool,
}

/// A hazard definition.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct HazardData {
    pub x: f32,
    pub y: f32,
    pub kind: HazardKind,
}

/// Kind of hazard.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum HazardKind {
    Spike,
    Saw { amplitude: f32, speed: f32 },
    Fire,
    FallingPlatform,
}

/// A patrolling enemy definition.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct MonsterData {
    pub x: f32,
    pub y: f32,
    pub start_x: f32,
    pub end_x: f32,
    pub speed: f32,
}

/// Full layout for a single level.
#[allow(dead_code)]
#[derive(Clone)]
pub struct LevelLayout {
    /// Ground platform: (center_x, center_y, width).
    pub ground: (f32, f32, f32),
    /// Platforms in the level.
    pub platforms:    [PlatformData; MAX_PLATFORMS],
    pub platform_n:    usize,
    /// Collectible positions: (x, y).
    pub collectibles:  [(f32, f32); MAX_COLLECTIBLES],
    pub collectible_n: usize,
    /// Hazards in the level.
    pub hazards:       [HazardData; MAX_HAZARDS],
    pub hazard_n:       usize,
    /// Monsters in the level.
    pub monsters:      [MonsterData; MAX_MONSTERS],
    pub monster_n:     usize,
    /// Goal flag position: (x, y).
    pub flag: (f32, f32),
    /// Right boundary for the player.
    pub right_bound: f32,
    /// Theme for this level.
    pub theme: LevelTheme,
}

// ─────────────────────────────────────────────────────────────────────────────
// Level registry
// ─────────────────────────────────────────────────────────────────────────────

/// All levels in order. Index 0 = Level 1, Index 1 = Level 2, etc.
pub const LEVELS: [LevelLayout; 7] = [
    // ── Level 1: Grassland ─────────────────────────────────────────────────────
    level_layout!(
        ground: 640.0, -320.0, 3200.0;
        platforms: [
            PlatformData { x: -200.0, y: -270.0, width: 200.0, tint: Color::WHITE, moving: None },
            PlatformData { x: 100.0,  y: -250.0, width: 180.0, tint: Color::WHITE, moving: None },
            PlatformData { x: 380.0,  y: -220.0, width: 220.0, tint: Color::WHITE, moving: None },
            PlatformData { x: 650.0,  y: -250.0, width: 160.0, tint: Color::WHITE, moving: None },
            PlatformData { x: 900.0,  y: -220.0, width: 240.0, tint: Color::WHITE, moving: None },
            PlatformData { x: 1150.0, y: -250.0, width: 160.0, tint: Color::WHITE, moving: None },
            PlatformData { x: 1400.0, y: -275.0, width: 200.0, tint: Color::WHITE, moving: None },
        ];
        collectibles: [
            (-300.0, -260.0), (-200.0, -220.0),
            (100.0,  -200.0), (380.0,  -170.0),
            (650.0,  -200.0), (900.0,  -170.0),
            (1150.0, -200.0), (1400.0, -225.0),
        ];
        hazards: [
            HazardData { x: 50.0,   y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 270.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 540.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 790.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 1060.0, y: -302.0, kind: HazardKind::Spike },
        ];
        monsters: [];
        flag: 1800.0, -278.0;
        bound: 1950.0;
        theme: LevelTheme::Grassland
    ),

    // ── Level 2: Rocky Mountain ───────────────────────────────────────────────
    level_layout!(
        ground: 900.0, -320.0, 3600.0;
        platforms: [
            PlatformData { x: -300.0, y: -255.0, width: 180.0, tint: Color::srgb(0.7, 0.9, 1.0), moving: None },
            PlatformData { x: 60.0,   y: -215.0, width: 160.0, tint: Color::srgb(0.7, 0.9, 1.0), moving: None },
            PlatformData { x: 700.0,  y: -245.0, width: 140.0, tint: Color::srgb(0.7, 0.9, 1.0), moving: None },
            PlatformData { x: 1310.0, y: -225.0, width: 160.0, tint: Color::srgb(0.7, 0.9, 1.0), moving: None },
            PlatformData { x: 1920.0, y: -240.0, width: 160.0, tint: Color::srgb(0.7, 0.9, 1.0), moving: None },
            PlatformData { x: 380.0,  y: -178.0, width: 160.0, tint: Color::srgb(0.4, 0.95, 0.9), moving: Some(MovingData { amplitude: 80.0, speed: 1.2, horizontal: true }) },
            PlatformData { x: 1000.0, y: -190.0, width: 160.0, tint: Color::srgb(0.4, 0.95, 0.9), moving: Some(MovingData { amplitude: 100.0, speed: 1.5, horizontal: true }) },
            PlatformData { x: 1620.0, y: -178.0, width: 160.0, tint: Color::srgb(0.4, 0.95, 0.9), moving: Some(MovingData { amplitude: 70.0, speed: 1.0, horizontal: true }) },
        ];
        collectibles: [
            (-300.0, -205.0), (60.0, -165.0),
            (380.0, -128.0), (700.0, -195.0),
            (1000.0, -140.0), (1310.0, -175.0),
            (1620.0, -128.0), (1920.0, -190.0),
        ];
        hazards: [
            HazardData { x: -80.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 230.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 560.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 870.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 1170.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 1660.0, y: -302.0, kind: HazardKind::Spike },
        ];
        monsters: [];
        flag: 2250.0, -278.0;
        bound: 2350.0;
        theme: LevelTheme::RockyMountain
    ),

    // ── Level 3: Water World ───────────────────────────────────────────────────
    level_layout!(
        ground: 1250.0, -320.0, 4800.0;
        platforms: [
            PlatformData { x: -250.0, y: -248.0, width: 160.0, tint: Color::srgb(0.3, 0.6, 0.9), moving: None },
            PlatformData { x: 580.0,  y: -158.0, width: 140.0, tint: Color::srgb(0.3, 0.6, 0.9), moving: None },
            PlatformData { x: 1380.0, y: -172.0, width: 140.0, tint: Color::srgb(0.3, 0.6, 0.9), moving: None },
            PlatformData { x: 2200.0, y: -155.0, width: 140.0, tint: Color::srgb(0.3, 0.6, 0.9), moving: None },
            PlatformData { x: 2640.0, y: -225.0, width: 140.0, tint: Color::srgb(0.3, 0.6, 0.9), moving: None },
            PlatformData { x: 200.0,  y: -200.0, width: 130.0, tint: Color::srgb(0.4, 0.75, 1.0), moving: Some(MovingData { amplitude: 70.0, speed: 1.1, horizontal: true }) },
            PlatformData { x: 1800.0, y: -215.0, width: 120.0, tint: Color::srgb(0.4, 0.75, 1.0), moving: Some(MovingData { amplitude: 90.0, speed: 1.4, horizontal: true }) },
            PlatformData { x: 1010.0, y: -225.0, width: 120.0, tint: Color::srgb(0.4, 0.9, 0.8), moving: Some(MovingData { amplitude: 65.0, speed: 0.9, horizontal: false }) },
        ];
        collectibles: [
            (-250.0, -198.0), (200.0, -150.0),
            (580.0,  -108.0), (1010.0, -160.0),
            (1380.0, -122.0), (1800.0, -165.0),
            (2200.0, -105.0), (2640.0, -175.0),
        ];
        hazards: [
            HazardData { x: 0.0,    y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 420.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 810.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 1220.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 1620.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 2020.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 2440.0, y: -302.0, kind: HazardKind::Spike },
        ];
        monsters: [];
        flag: 2950.0, -278.0;
        bound: 3050.0;
        theme: LevelTheme::WaterWorld
    ),

    // ── Level 4: Fiery Furnace ─────────────────────────────────────────────────
    level_layout!(
        ground: 1900.0, -320.0, 6400.0;
        platforms: [
            PlatformData { x: -300.0,  y: -248.0, width: 200.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 150.0,   y: -215.0, width: 160.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 650.0,   y: -180.0, width: 150.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 1100.0,  y: -168.0, width: 140.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 1630.0,  y: -192.0, width: 140.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 2060.0,  y: -172.0, width: 140.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 2560.0,  y: -188.0, width: 140.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 3110.0,  y: -168.0, width: 150.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 3620.0,  y: -202.0, width: 200.0, tint: Color::srgb(0.80, 0.38, 0.18), moving: None },
            PlatformData { x: 420.0,   y: -198.0, width: 110.0, tint: Color::srgb(1.0, 0.48, 0.18), moving: Some(MovingData { amplitude: 90.0, speed: 2.0, horizontal: true }) },
            PlatformData { x: 870.0,   y: -202.0, width: 110.0, tint: Color::srgb(1.0, 0.48, 0.18), moving: Some(MovingData { amplitude: 80.0, speed: 2.2, horizontal: true }) },
            PlatformData { x: 1365.0,  y: -188.0, width: 110.0, tint: Color::srgb(1.0, 0.48, 0.18), moving: Some(MovingData { amplitude: 70.0, speed: 1.8, horizontal: true }) },
            PlatformData { x: 1850.0,  y: -172.0, width: 100.0, tint: Color::srgb(1.0, 0.48, 0.18), moving: Some(MovingData { amplitude: 65.0, speed: 2.5, horizontal: true }) },
            PlatformData { x: 2315.0,  y: -168.0, width: 100.0, tint: Color::srgb(0.4, 1.0, 0.35), moving: Some(MovingData { amplitude: 55.0, speed: 1.4, horizontal: false }) },
            PlatformData { x: 2830.0,  y: -182.0, width: 100.0, tint: Color::srgb(1.0, 0.48, 0.18), moving: Some(MovingData { amplitude: 75.0, speed: 2.0, horizontal: true }) },
            PlatformData { x: 3355.0,  y: -178.0, width: 100.0, tint: Color::srgb(1.0, 0.48, 0.18), moving: Some(MovingData { amplitude: 78.0, speed: 2.3, horizontal: true }) },
            PlatformData { x: -80.0,   y: -268.0, width: 96.0, tint: Color::srgb(0.95, 0.72, 0.25), moving: None },
            PlatformData { x: 560.0,   y: -228.0, width: 96.0, tint: Color::srgb(0.95, 0.72, 0.25), moving: None },
            PlatformData { x: 1920.0,  y: -228.0, width: 96.0, tint: Color::srgb(0.95, 0.72, 0.25), moving: None },
        ];
        collectibles: [
            (-300.0, -218.0), (150.0, -185.0),
            (420.0, -148.0), (1100.0, -138.0),
            (1630.0, -162.0), (2315.0, -108.0),
            (3110.0, -138.0), (3620.0, -172.0),
        ];
        hazards: [
            HazardData { x: 280.0,  y: -245.0, kind: HazardKind::Saw { amplitude: 60.0, speed: 3.0 } },
            HazardData { x: 760.0,  y: -228.0, kind: HazardKind::Saw { amplitude: 70.0, speed: 3.5 } },
            HazardData { x: 1480.0, y: -225.0, kind: HazardKind::Saw { amplitude: 65.0, speed: 3.2 } },
            HazardData { x: 2700.0, y: -222.0, kind: HazardKind::Saw { amplitude: 60.0, speed: 3.8 } },
            HazardData { x: 50.0,   y: -278.0, kind: HazardKind::Fire },
            HazardData { x: 345.0,  y: -278.0, kind: HazardKind::Fire },
            HazardData { x: 720.0,  y: -278.0, kind: HazardKind::Fire },
            HazardData { x: 1200.0, y: -278.0, kind: HazardKind::Fire },
            HazardData { x: 1755.0, y: -278.0, kind: HazardKind::Fire },
            HazardData { x: 2450.0, y: -278.0, kind: HazardKind::Fire },
            HazardData { x: 3010.0, y: -278.0, kind: HazardKind::Fire },
            HazardData { x: -100.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 228.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 512.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 988.0,  y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 1448.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 2112.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 2688.0, y: -302.0, kind: HazardKind::Spike },
            HazardData { x: 3258.0, y: -302.0, kind: HazardKind::Spike },
        ];
        monsters: [
            MonsterData { x: 625.0,  y: -146.0, start_x: 568.0,  end_x: 732.0,  speed: 88.0 },
            MonsterData { x: 2040.0, y: -138.0, start_x: 1968.0, end_x: 2142.0, speed: 96.0 },
            MonsterData { x: 3090.0, y: -134.0, start_x: 3018.0, end_x: 3192.0, speed: 92.0 },
        ];
        flag: 3860.0, -278.0;
        bound: 4000.0;
        theme: LevelTheme::FieryFurnace
    ),

    // ── Level 5: Frigid Freezer ────────────────────────────────────────────────
    level_layout!(
        ground: 900.0, -320.0, 3600.0;
        platforms: [];
        collectibles: [];
        hazards: [];
        monsters: [];
        flag: 2250.0, -278.0;
        bound: 2350.0;
        theme: LevelTheme::FrigidFreezer
    ),

    // ── Level 6: Electric Star ──────────────────────────────────────────────────
    level_layout!(
        ground: 900.0, -320.0, 3600.0;
        platforms: [];
        collectibles: [];
        hazards: [];
        monsters: [];
        flag: 2250.0, -278.0;
        bound: 2350.0;
        theme: LevelTheme::ElectricStar
    ),

    // ── Level 7: Boss ───────────────────────────────────────────────────────────
    level_layout!(
        ground: 900.0, -320.0, 3600.0;
        platforms: [];
        collectibles: [];
        hazards: [];
        monsters: [];
        flag: 2250.0, -278.0;
        bound: 2350.0;
        theme: LevelTheme::Boss
    ),
];

// ─────────────────────────────────────────────────────────────────────────────
// Spawn logic
// ─────────────────────────────────────────────────────────────────────────────

/// Spawn all game entities for a level from its layout.
pub fn spawn_level_layout(
    commands: &mut Commands,
    layout: &LevelLayout,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) {
    use bevy::prelude::*;
    use crate::components::{
        Collider, FallingPlatform, GameEntity, Hazard, MovingHazard, MovingPlatform, Platform,
    };

    // Pixel Adventure terrain sheet: 352x176 = 22 cols x 11 rows of 16x16 tiles.
    // Grass platform tiles start at row 0, col 5.
    let ground_tiles_x = (layout.ground.2 / 16.0).ceil() as usize;
    let terrain_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 22, 11, None, None);
    let terrain_handle = layouts.add(terrain_layout);
    let terrain_img: Handle<Image> = asset_server.load("Pixel Adventure 1/Free/Terrain/Terrain (16x16).png");
    let grass_left = 5;
    let grass_mid = 6;
    let grass_right = 8;
    let yellow_left = 198;
    let yellow_mid = 199;
    let yellow_right = 201;
    
    let (gx, gy, gw) = layout.ground;
    
    // Spawn ground as a row of tiled sprites
    for i in 0..ground_tiles_x {
        let tile_x = gx - gw/2.0 + (i as f32 * 16.0) + 8.0;
        commands.spawn((
            GameEntity,
            Platform,
            Transform::from_xyz(tile_x, gy, 0.1),
            Sprite {
                image: terrain_img.clone(),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                texture_atlas: Some(TextureAtlas {
                    layout: terrain_handle.clone(),
                    index: if i == 0 {
                        grass_left
                    } else if i == ground_tiles_x - 1 {
                        grass_right
                    } else {
                        grass_mid
                    },
                }),
                ..default()
            },
            Collider { half_w: 8.0, half_h: PLATFORM_H / 2.0 },
        ));
    }

    // Floating platforms use the same Pixel Adventure terrain sheet so the world reads as one tileset.
    for i in 0..layout.platform_n {
        let p = layout.platforms[i];
        let tiles_x = (p.width / 16.0).ceil() as usize;
        let is_moving = p.moving.is_some();
        
        for j in 0..tiles_x {
            let tile_x = p.x - p.width/2.0 + (j as f32 * 16.0) + 8.0;
            let tile_idx = if is_moving {
                if j == 0 { yellow_left } else if j == tiles_x - 1 { yellow_right } else { yellow_mid }
            } else if j == 0 {
                grass_left
            } else if j == tiles_x - 1 {
                grass_right
            } else {
                grass_mid
            };
            let entity = commands.spawn((
                GameEntity,
                Platform,
                Transform::from_xyz(tile_x, p.y, 0.1),
                Sprite {
                    image: terrain_img.clone(),
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    texture_atlas: Some(TextureAtlas {
                        layout: terrain_handle.clone(),
                        index: tile_idx,
                    }),
                    ..default()
                },
                Collider { half_w: 8.0, half_h: PLATFORM_H / 2.0 },
            )).id();
            if let Some(moving) = p.moving {
                commands.entity(entity).insert(MovingPlatform {
                    start_x: tile_x,
                    start_y: p.y,
                    amplitude: moving.amplitude,
                    speed: moving.speed,
                    horizontal: moving.horizontal,
                    elapsed: 0.0,
                    delta: Vec2::ZERO,
                });
            }
        }
    }

    // Collectibles use Pixel Adventure fruit to fit the tileset.
    let fruit_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 17, 1, None, None);
    let fruit_handle = layouts.add(fruit_layout);
    let fruit_img: Handle<Image> = asset_server.load("Pixel Adventure 1/Free/Items/Fruits/Cherries.png");

    for i in 0..layout.collectible_n {
        let (cx, cy) = layout.collectibles[i];
        commands.spawn((
            GameEntity,
            crate::components::Collectible,
            Transform::from_xyz(cx, cy, 0.0),
            Sprite {
                image: fruit_img.clone(),
                custom_size: Some(Vec2::splat(32.0)),
                texture_atlas: Some(TextureAtlas {
                    layout: fruit_handle.clone(),
                    index: 0,
                }),
                ..default()
            },
            Collider { half_w: 16.0, half_h: 16.0 },
        ));
    }

    // Pixel Adventure traps.
    let spike_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 1, None, None);
    let spike_handle = layouts.add(spike_layout);
    let spike_img: Handle<Image> = asset_server.load("Pixel Adventure 1/Free/Traps/Spikes/Idle.png");
    let saw_layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::new(38, 38), 8, 1, None, None));
    let saw_img: Handle<Image> = asset_server.load("Pixel Adventure 1/Free/Traps/Saw/On (38x38).png");
    let fire_layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::new(16, 32), 3, 1, None, None));
    let fire_img: Handle<Image> = asset_server.load("Pixel Adventure 1/Free/Traps/Fire/On (16x32).png");
    let falling_layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::new(32, 10), 4, 1, None, None));
    let falling_img: Handle<Image> = asset_server.load("Pixel Adventure 1/Free/Traps/Falling Platforms/On (32x10).png");

    for i in 0..layout.hazard_n {
        let h = layout.hazards[i];
        match h.kind {
            HazardKind::Spike => {
                commands.spawn((
                    GameEntity,
                    Hazard,
                    Transform::from_xyz(h.x, h.y, 0.2),
                    Sprite {
                        image: spike_img.clone(),
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        texture_atlas: Some(TextureAtlas {
                            layout: spike_handle.clone(),
                            index: 0,
                        }),
                        ..default()
                    },
                    Collider { half_w: 12.0, half_h: 12.0 },
                ));
            }
            HazardKind::Saw { amplitude, speed } => {
                commands.spawn((
                    GameEntity,
                    Hazard,
                    MovingHazard {
                        start_x: h.x,
                        amplitude,
                        speed,
                        elapsed: 0.0,
                    },
                    Transform::from_xyz(h.x, h.y, 0.2),
                    Sprite {
                        image: saw_img.clone(),
                        custom_size: Some(Vec2::splat(38.0)),
                        texture_atlas: Some(TextureAtlas {
                            layout: saw_layout.clone(),
                            index: 0,
                        }),
                        ..default()
                    },
                    Collider { half_w: 17.0, half_h: 17.0 },
                ));
            }
            HazardKind::Fire => {
                commands.spawn((
                    GameEntity,
                    Hazard,
                    Transform::from_xyz(h.x, h.y, 0.2),
                    Sprite {
                        image: fire_img.clone(),
                        custom_size: Some(Vec2::new(32.0, 64.0)),
                        texture_atlas: Some(TextureAtlas {
                            layout: fire_layout.clone(),
                            index: 0,
                        }),
                        ..default()
                    },
                    Collider { half_w: 12.0, half_h: 24.0 },
                ));
            }
            HazardKind::FallingPlatform => {
                commands.spawn((
                    GameEntity,
                    Platform,
                    FallingPlatform {
                        original_x: h.x,
                        warned: false,
                        timer: 0.6,
                        falling: false,
                        fall_velocity: 0.0,
                    },
                    Transform::from_xyz(h.x, h.y, 0.1),
                    Sprite {
                        image: falling_img.clone(),
                        custom_size: Some(Vec2::new(96.0, 30.0)),
                        texture_atlas: Some(TextureAtlas {
                            layout: falling_layout.clone(),
                            index: 0,
                        }),
                        ..default()
                    },
                    Collider { half_w: 48.0, half_h: 15.0 },
                ));
            }
        }
    }

    // Goal checkpoint from Pixel Adventure.
    let goal_layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(64), 1, 1, None, None));
    let goal_img: Handle<Image> = asset_server.load("Pixel Adventure 1/Free/Items/Checkpoints/End/End (Idle).png");
    commands.spawn((
        GameEntity,
        crate::components::GoalFlag,
        Transform::from_xyz(layout.flag.0, layout.flag.1, 0.2),
        Sprite {
            image: goal_img,
            custom_size: Some(Vec2::new(64.0, 64.0)),
            texture_atlas: Some(TextureAtlas {
                layout: goal_layout,
                index: 0,
            }),
            ..default()
        },
    ));
}

// Platform height constant used by spawn logic.
const PLATFORM_H: f32 = 20.0;
