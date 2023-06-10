use std::fmt;

use crate::libs::user_app::structures::models::User;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "User<{id}>{{`username`= {username}, `email`= {email}, `password`= {val}, `date_of_birth`= {date_of_birth} , `gender`= {gender}}}" , 
            id=self.id, 
            username=self.username, 
            email=self.email, 
            val = self.password(),
            date_of_birth=self.date_of_birth, 
            gender=self.gender
        )
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "User<{id}>{{`username`= {username}, `email`= {email}, `password`= {val}, `date_of_birth`= {date_of_birth} , `gender`= {gender}}}" , 
            id=self.id, 
            username=self.username, 
            email=self.email, 
            val = self.password(),
            date_of_birth=self.date_of_birth, 
            gender=self.gender
        )
    }
}