use crate::models::AdminRoles as ThisModel;
use super::Controller;
//use fluffy::{ model::Model, };

pub struct AdminRoles { }

impl Controller<ThisModel> for AdminRoles { 

    type M = ThisModel;

    //fn edit_for_update(id: usize) -> Self::M { 
    //    row_for_update!(ThisModel, id, [
    //        name => String, 
    //        remark => String,
    //    ])
    //}
}
