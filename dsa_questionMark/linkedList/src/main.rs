
pub struct node{
    pub data : u64,
    pub next : Box<node>,
}

impl node{
    pub fn new(key: u64)-> node {
        node{key, None}
    }        
    pub fn append(key: u64, head: node)->(){
        let temp = head;
        while temp!=None {
            temp = temp.next;
        }
        temp.next = node :: new(key);
    }
    pub fn display(head: node)->(){
        let temp = head;
        while temp!=None {
            println!("{}, " temp.data);
            temp = temp.next;
        }
    }
}

fn main(){
    let head = None;
    node::append(6, head);
    node::display(head);

}
