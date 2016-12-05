pub trait NumPad {
    fn move_right(&mut self);
    fn move_left(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);

    fn current_digit(&self) -> char;
}
