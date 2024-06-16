
struct Node {
    value:i32,
    next:Option<Box<Node>>
}

struct LinkList{
    head:Option<Box<Node>>
}

impl LinkList {
    fn new() -> Self {
        LinkList {head:None}
    }

    fn append(&mut self,value:i32){
        let new_node = Box::new(Node {value,next:None});

        match self.head{
            Some(ref mut node) =>{
                let mut current = node;

                while let Some(ref mut next_node) = current.next{
                    current = next_node;
                }

                current.next = Some(new_node);
            },
            None => {
                self.head = Some(new_node);
            }
        }
    }

    fn delete_node(&mut self,target:i32) {

        if let Some(ref mut head) = self.head{
            if head.value == target{
                self.head = head.next.take();
                return;
            };
        };

        let mut current = &mut self.head;

        while let Some(ref mut  node) = current  {
            if let Some(ref mut next_node) = node.next {
                if next_node.value == target {
                    node.next = next_node.next.take();
                    return;
                }

            }
            current = &mut node.next;
        }
       
    }

    fn search_node(&self,target:i32) -> Option<i32>{
        let mut current = &self.head;

        while let Some(ref node) = current  {
            if node.value == target{
                return Some(node.value);
            }
            current = &node.next;
        };
        None
    }

    

    fn traverse(&self){
        let mut current = &self.head;

        while let Some(node) = current  {
            println!("{}",node.value);
            current = &node.next
        }
    }

    fn ll_len(&self) -> i32{
        let mut current = &self.head;
        let mut count = 0;

        while let Some(ref node) = current {
            count += 1;
            current = &node.next;
        }


        count
        
    }

    fn reverse_ll(&mut self) {
       let mut current = self.head.take();
       let mut prev: Option<Box<Node>> = None;

       while let Some(mut curr) = current {
           let next = curr.next.take();
           curr.next = prev;
           prev = Some(curr);
           current = next;
       } 

       self.head = prev;

        
    }
}





fn main() {
    let mut list = LinkList::new();
    list.append(1);
    list.append(2);
    list.append(3000);
    list.append(11);
    list.append(22);
    list.append(30002);

    list.traverse();

    // println!("The length of link list is {}",list.ll_len());
    list.reverse_ll();

    list.traverse();


}
