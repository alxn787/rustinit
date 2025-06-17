
fn main (){
    let str1 = String::from("alen");
    let ans:String;

    {
        let str2 = String::from("alex");
        ans = get_longer_string(&str1, &str2).to_string();
    }
    print!("{}", ans);

}

fn get_longer_string<'a, 'b>(s1:&'a String, s2:&'b String)-> &'b String{ 
    if s1.len()>s2.len(){
        return &s1;
    }
    else {return &s2;}
}

