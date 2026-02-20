struct Branches{
    src_branch: String,
    dest_branch: String,
}

impl Branches{
   fn init(branch_1: &str, branch_2: &str) -> Self{
       Branches{
           src_branch: branch_1.to_string(),
           dest_branch: branch_2.to_string(),
       }
   } 
}
