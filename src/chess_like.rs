use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawSquare<PieceType, ColorType> {
    pub data: Option<(PieceType, ColorType)>,
}

pub trait GenericRank<StorageType: GenericStorage>: Copy + Clone + PartialEq + Eq {
    fn to_storage(self) -> StorageType;
    fn from_storage(input: StorageType) -> Self;
}

pub trait GenericFile<StorageType: GenericStorage>: Copy + Clone + PartialEq + Eq {
    fn to_storage(self) -> StorageType;
    fn from_storage(input: StorageType) -> Self;
}

pub trait GenericStorage:
    Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Add + AddAssign + Mul + Div + Rem
{
}

impl GenericStorage for u8 {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Move<BoardType: GenericBoard>
{
    pub src: SquarePos<Self>,
    pub dest: SquarePos<Self>,
}

pub trait GenericPiece: PartialEq + Eq + Copy {}

pub trait GenericColor: PartialEq + Eq + Copy {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefaultColorScheme {
    While,
    Black,
}

impl GenericColor for DefaultColorScheme {}

//Beefy structs

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SquarePos<BoardType: GenericBoard> {
    pos: BoardType::StorageType,
}

#[derive(new)]
pub struct SquareIter<BoardType: GenericBoard> {
    current: SquarePos<BoardType>,
    max_size: SquarePos<BoardType>,
}

pub trait GenericBoard: Sized + Copy + Clone + PartialEq + Eq {
    type PieceType: GenericPiece;
    type ColorType: GenericColor;
    type StorageType: GenericStorage;
    type FileType: GenericFile<Self::StorageType>;
    type RankType: GenericRank<Self::StorageType>;
    type RawMoveIteratorType: Iterator<Item = Move<Self>>;

    fn side_len() -> Self::StorageType;
    ///Creates an empty board
    fn new() -> Self;

    ///Creates a board with pieces placed in their default positions
    fn default() -> Self;

    fn is_move_legal(&self, board_move: Move<Self>) -> bool;

    ///Enumerates the 'raw' moves using the movement rules for the piece occupying the requested
    ///square. Raw means the list may contain moves that transitively are illegal because they
    ///cause checks.
    fn raw_moves_for_piece(&self, pos: SquarePos<Self>) -> Self::RawMoveIteratorType;

    ///Returns a list of the locations of the pieces that attack a square. Attacking is defined as
    ///having a legal move that moves takes a potential attacker its starting position to pos
    fn get_attackers_of_square(&self, target_pos: SquarePos<Self>) -> Vec<SquarePos<Self>>;

    fn raw_square_iter(&self) -> SquareIter<Self>;

    fn get(&self, pos: SquarePos<Self>) -> &RawSquare<Self::PieceType, Self::ColorType>;

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(&self, pos: SquarePos<Self>, piece: &mut RawSquare<Self::PieceType, Self::ColorType>);

    fn set(
        &mut self,
        pos: SquarePos<Self>,
        piece: RawSquare<Self::PieceType, Self::ColorType>,
    ) -> RawSquare<Self::PieceType, Self::ColorType>;
}

impl<BoardType> SquarePos<BoardType>
where
    BoardType: GenericBoard,
    BoardType::StorageType: GenericStorage,
{
    fn from_raw(pos: BoardType::StorageType) -> SquarePos<BoardType> {
        SquarePos { pos }
    }

    fn new(file: BoardType::FileType, rank: BoardType::RankType) -> SquarePos<BoardType> {
        SquarePos {
            pos: file.to_storage() * Self::BoardType::side_len(),
        }
    }

    fn rank(&self) -> BoardType::RankType {
        return self.pos % Self::BoardType::side_len();
    }

    fn file(&self) -> BoardType::FileType {
        return self.pos / Self::BoardType::side_len();
    }

    fn raw_value(&self) -> BoardType::StorageType {
        self.pos
    }

    fn side_len() -> BoardType::StorageType {
        BoardType::side_len()
    }
}

impl<BoardType: GenericBoard> Iterator for SquareIter<BoardType> {
    type Item = SquarePos<BoardType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max_size {
            None
        } else {
            let result = self.current;
            self.current += 1;
            Some(SquarePos::from_raw(result))
        }
    }
}

impl<PieceType, ColorType> RawSquare<PieceType, ColorType> {
    pub fn empty() -> RawSquare<PieceType, ColorType> {
        RawSquare { data: None }
    }

    pub fn new(piece: PieceType, color: ColorType) -> RawSquare<PieceType, ColorType> {
        RawSquare {
            data: Some((piece, color)),
        }
    }
}
