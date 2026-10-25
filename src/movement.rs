//! Shared movement rules for lessons and capture-only matches. King moves are used only by the opponent in checkmate matches.
use crate::rules::{Board, Square};
pub const KINDS: [&str; 6] = ["rook", "bishop", "knight", "pawn", "queen", "king"];
pub fn value(kind: &str) -> i32 {
    match kind {
        "queen" => 9,
        "rook" => 5,
        "bishop" | "knight" => 3,
        "pawn" => 1,
        _ => 0,
    }
}
pub fn legal(
    kind: &str,
    from: Square,
    to: Square,
    board: &Board,
    forward: i16,
    occupied: impl Fn(Square) -> Option<bool>,
) -> bool {
    if from == to || to.x >= board.width || to.y >= board.height || occupied(to) == Some(true) {
        return false;
    }
    let dx = to.x as i16 - from.x as i16;
    let dy = to.y as i16 - from.y as i16;
    if kind == "pawn" {
        return dy == forward
            && ((dx == 0 && occupied(to).is_none())
                || (dx.abs() == 1 && occupied(to) == Some(false)));
    }
    if kind == "king" {
        return dx.abs() <= 1 && dy.abs() <= 1;
    }
    if kind == "knight" {
        return (dx.abs() == 1 && dy.abs() == 2) || (dx.abs() == 2 && dy.abs() == 1);
    }
    let direction = match kind {
        "rook" => dx == 0 || dy == 0,
        "bishop" => dx.abs() == dy.abs(),
        "queen" => dx == 0 || dy == 0 || dx.abs() == dy.abs(),
        _ => false,
    };
    if !direction {
        return false;
    }
    let (mut x, mut y) = (from.x as i16 + dx.signum(), from.y as i16 + dy.signum());
    while (x, y) != (to.x as i16, to.y as i16) {
        if occupied(Square {
            x: x as u8,
            y: y as u8,
        })
        .is_some()
        {
            return false;
        }
        x += dx.signum();
        y += dy.signum();
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    fn s(v: &str) -> Square {
        Square::parse(v).unwrap()
    }
    #[test]
    fn pawns_move_forward_only_capture_diagonally_and_never_jump() {
        let b = Board {
            width: 5,
            height: 5,
        };
        assert!(legal("pawn", s("b2"), s("b3"), &b, 1, |_| None));
        assert!(!legal("pawn", s("b2"), s("b4"), &b, 1, |_| None));
        assert!(!legal("pawn", s("b2"), s("a3"), &b, 1, |_| None));
        assert!(legal("pawn", s("b2"), s("a3"), &b, 1, |q| (q == s("a3"))
            .then_some(false)));
        assert!(!legal("pawn", s("b2"), s("b3"), &b, 1, |q| (q == s("b3"))
            .then_some(false)));
        assert!(!legal("pawn", s("b2"), s("b1"), &b, 1, |_| None));
        assert!(legal("pawn", s("b3"), s("b2"), &b, -1, |_| None));
        assert!(!legal("pawn", s("b5"), s("b4"), &b, 1, |_| None));
    }
    #[test]
    fn queens_combine_rook_and_bishop_paths_but_cannot_jump_or_capture_allies() {
        let b = Board {
            width: 5,
            height: 5,
        };
        for to in ["a5", "e1", "e5"] {
            assert!(legal("queen", s("a1"), s(to), &b, 1, |_| None));
        }
        assert!(!legal("queen", s("a1"), s("b3"), &b, 1, |_| None));
        assert!(!legal("queen", s("a1"), s("e5"), &b, 1, |q| (q == s("c3"))
            .then_some(false)));
        assert!(!legal("queen", s("a1"), s("e1"), &b, 1, |q| (q == s("e1"))
            .then_some(true)));
    }
}
