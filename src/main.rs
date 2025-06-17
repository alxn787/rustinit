use borsh:: {BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize,BorshDeserialize,Clone,Debug)]
struct User {
    username: String,
    password: String
}

fn main (){
    let u = User{
        username: String::from("alen"),
        password: String::from("password")
    };

    let mut v: Vec<u8> = Vec::new();

    let _ans = u.serialize(&mut v);

    let user = User::try_from_slice(&mut v);
    print!("{:?}", v);
    print!("{:?}", user)
}

