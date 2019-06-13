use *;

/// Tournament permissions iterator
pub struct PermissionsIter<'a> {
    client: &'a Toornament,

    /// Fetch permissions of the following tournament id
    tournament_id: TournamentId,
}
impl<'a> PermissionsIter<'a> {
    /// Create new permissions iter
    pub fn new(client: &'a Toornament, tournament_id: TournamentId) -> PermissionsIter {
        PermissionsIter {
            client: client,
            tournament_id: tournament_id,
        }
    }
}

/// Modifiers
impl<'a> PermissionsIter<'a> {
    /// A permission with id
    pub fn with_id(self, id: PermissionId) -> PermissionIter<'a> {
        PermissionIter {
            client: self.client,
            tournament_id: self.tournament_id,
            id: id,
        }
    }

    /// Create a permission
    pub fn create<F: 'static + FnMut() -> Permission>(self, creator: F) -> PermissionCreator<'a> {
        PermissionCreator {
            client: self.client,
            tournament_id: self.tournament_id,
            creator: Box::new(creator),
        }
    }
}

/// Terminators
impl<'a> PermissionsIter<'a> {
    /// Collects the permissions
    pub fn collect<T: From<Permissions>>(self) -> Result<T> {
        Ok(T::from(
            self.client.tournament_permissions(self.tournament_id)?,
        ))
    }
}

/// Tournament permission iterator
pub struct PermissionIter<'a> {
    client: &'a Toornament,

    /// Fetch permissions of the following tournament id
    tournament_id: TournamentId,
    /// Fetch permission with id
    id: PermissionId,
}
impl<'a> PermissionIter<'a> {
    /// Create new permission iter
    pub fn new(
        client: &'a Toornament,
        tournament_id: TournamentId,
        id: PermissionId,
    ) -> PermissionIter {
        PermissionIter {
            client: client,
            tournament_id: tournament_id,
            id: id,
        }
    }
}

/// Modifiers
impl<'a> PermissionIter<'a> {
    /// Fetch a permission with the following id
    pub fn with_id(self, id: PermissionId) -> PermissionIter<'a> {
        PermissionIter {
            client: self.client,
            tournament_id: self.tournament_id,
            id: id,
        }
    }

    // TODO
    /* There is no ability to edit permissions yet
    /// Edit a permission
    pub fn edit<F: 'static + FnMut(Permission) -> Permission>(self, editor: F)
        -> PermissionEditor<'a> {
        PermissionEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            id: self.id,
            editor: Box::new(editor),
        }
    }
    */

    /// Fetch permission attributes
    pub fn attributes(self) -> PermissionAttributesIter<'a> {
        PermissionAttributesIter {
            client: self.client,
            tournament_id: self.tournament_id,
            permission_id: self.id,
        }
    }
}

/// Terminators
impl<'a> PermissionIter<'a> {
    /// Fetch the permission
    pub fn collect<T: From<Permission>>(self) -> Result<T> {
        Ok(T::from(
            self.client
                .tournament_permission(self.tournament_id, self.id)?,
        ))
    }

    /// Delete this permission
    pub fn delete(self) -> Result<()> {
        self.client
            .delete_tournament_permission(self.tournament_id, self.id)
    }
}

/// A lazy permission creator
pub struct PermissionCreator<'a> {
    client: &'a Toornament,

    /// A tournament to which the permission will belong to
    tournament_id: TournamentId,
    /// Permission creator
    creator: Box<FnMut() -> Permission>,
}

/// Terminators
impl<'a> PermissionCreator<'a> {
    /// Creates the permission
    pub fn update(mut self) -> Result<Permission> {
        self.client
            .create_tournament_permission(self.tournament_id, (self.creator)())
    }

    /// Create and return iter
    pub fn update_iter(mut self) -> Result<PermissionIter<'a>> {
        let created = self
            .client
            .create_tournament_permission(self.tournament_id.clone(), (self.creator)())?;

        match created.id {
            Some(id) => Ok(PermissionIter::new(self.client, self.tournament_id, id)),
            None => Err(Error::Iter(IterError::NoPermissionId)),
        }
    }
}

// TODO
/* There is no ability to edit permissions yet
/// A lazy permission editor
pub struct PermissionEditor<'a> {
    client: &'a Toornament,

    /// A tournament to which the permission will belong to
    tournament_id: TournamentId,
    /// A permission to edit
    id: PermissionId,
    /// Permission creator
    editor: Box<FnMut(Permission) -> Permission>,
}

/// Terminators
impl<'a> PermissionEditor<'a> {
    /// Edits the permission
    pub fn update(mut self) -> Result<Permission> {
        // self.client.create_tournament_permission(self.tournament_id, (self.editor)())

        let original = match self.client.tournaments(Some(self.id), self.with_streams)?.0.first() {
            Some(t) => t.to_owned(),
            None => return Err(Error::Other("No such tournament")),
        };
        let edited = (self.editor)(original);
        self.client.edit_tournament(edited)
    }

    /// Edit and return iter
    pub fn update_iter(mut self) -> Result<PermissionIter<'a>> {
        let created = self.client.create_tournament_permission(self.tournament_id.clone(),
                                                               (self.editor)())?;

        match created.id {
            Some(id) => Ok(PermissionIter::new(self.client, self.tournament_id, id)),
            None => Err(Error::Other("Permission does not have an id")),
        }
    }
}
*/

/// A permission attributes iterator
pub struct PermissionAttributesIter<'a> {
    client: &'a Toornament,

    /// A tournament to which the permission will belong to
    tournament_id: TournamentId,
    /// A permission to edit
    permission_id: PermissionId,
}

/// Terminators
impl<'a> PermissionAttributesIter<'a> {
    /// Fetch the attributes
    pub fn collect<T: From<PermissionAttributes>>(self) -> Result<T> {
        Ok(T::from(
            self.client
                .tournament_permission(self.tournament_id, self.permission_id)?
                .attributes,
        ))
    }

    /// Edit the permission attributes
    pub fn edit<F: 'static + FnMut(PermissionAttributes) -> PermissionAttributes>(
        self,
        editor: F,
    ) -> PermissionAttributesEditor<'a> {
        PermissionAttributesEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            permission_id: self.permission_id,
            editor: Box::new(editor),
        }
    }

    /// Return permission for this attributes
    pub fn permission(self) -> PermissionIter<'a> {
        PermissionIter {
            client: self.client,
            tournament_id: self.tournament_id,
            id: self.permission_id,
        }
    }
}

/// A lazy permission attributes editor
pub struct PermissionAttributesEditor<'a> {
    client: &'a Toornament,

    /// A tournament to which the permission will belong to
    tournament_id: TournamentId,
    /// A permission to edit
    permission_id: PermissionId,
    /// Permission attributes editor
    editor: Box<FnMut(PermissionAttributes) -> PermissionAttributes>,
}

/// Terminators
impl<'a> PermissionAttributesEditor<'a> {
    /// Edits and the permission attributes
    pub fn update(mut self) -> Result<Permission> {
        let original = self
            .client
            .tournament_permission(self.tournament_id.clone(), self.permission_id.clone())?
            .attributes;
        let edited = (self.editor)(original);
        self.client.update_tournament_permission_attributes(
            self.tournament_id,
            self.permission_id,
            edited,
        )
    }

    /// Edit and return iter
    pub fn update_iter(mut self) -> Result<PermissionAttributesIter<'a>> {
        let original = self
            .client
            .tournament_permission(self.tournament_id.clone(), self.permission_id.clone())?
            .attributes;
        let edited = (self.editor)(original);
        let _ = self.client.update_tournament_permission_attributes(
            self.tournament_id.clone(),
            self.permission_id.clone(),
            edited,
        )?;
        Ok(PermissionAttributesIter {
            client: self.client,
            tournament_id: self.tournament_id,
            permission_id: self.permission_id,
        })
    }
}
