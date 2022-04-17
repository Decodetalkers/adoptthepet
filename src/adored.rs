pub trait Adored {
    fn disadored(&mut self);
    fn changeadmin(&mut self, admin: String);
    fn name(&self) -> String;
}
