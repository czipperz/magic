#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Permission {
    Attack,
    Block,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permissions {
    permissions: Vec<Permission>,
}

impl Permissions {
    pub fn add(&mut self, p: Permission) {
        self.permissions.push(p);
    }

    pub fn remove(&mut self, p: &Permission) {
        self.permissions
            .iter()
            .position(|x| *x == *p)
            .map(|pos| self.permissions.remove(pos));
    }

    pub fn allowed(&self, p: &Permission) -> bool {
        self.permissions.contains(p)
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions {
            permissions: vec![Permission::Attack, Permission::Block],
        }
    }
}
