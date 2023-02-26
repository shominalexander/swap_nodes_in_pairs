#[derive(Clone)]
struct Item { next: Option<Box<Item>>, value: i32 }

enum Pop { Last(i32)
         , Next(i32)
         }

impl Item {
 fn new(mut vector: Vec<i32>) -> Option<Item> {
  if vector.len() > 0 {
   let mut item: Item = Item { next: None, value: 0 };

   vector.sort();

   for (index, vector_item) in vector.into_iter().rev().enumerate() {
    if index == 0 {
     item.next = None;

    } else {//if index == 0 {
     item.next = Some(Box::new(Item { next: item.next.take(), value: item.value }));

    }//} else {//if index == 0 {

    item.value = vector_item;
   }//for (index, vector_item) in vector.into_iter().rev().enumerate() {

   Some(item)

  } else {//if vector.len() > 0 {
   None

  }//} else {//if vector.len() > 0 {
 }//fn new(mut vector: Vec<i32>) -> Option<Item> {

 fn pop(&mut self) -> Pop {
  let value: i32 = self.value;

  if let Some(mut rest) = self.next.take() {
   self.next = rest.next.take();

   self.value = rest.value;

   Pop::Next(value)

  } else {//if let Some(mut rest) = self.next.take() {
   Pop::Last(value)

  }//} else {//if let Some(mut rest) = self.next.take() {
 }//fn pop(&mut self) -> Pop {

 fn push(&mut self, value: i32) {
  self.next = Some(Box::new(Item { next: self.next.take(), value: self.value }));

  self.value = value;
 }//fn push(&mut self, value: i32) {

 fn vector(mut item: Item) -> Vec<i32> {
  let mut vector: Vec<i32> = Vec::new();

  vector.push(item.value);

  while let Some(rest) = item.next {
   vector.push(rest.value);

   item.next = rest.next;
  }//while let Some(rest) = item.next {

  vector
 }//fn vector(mut item: Item) -> Vec<i32> {
}//impl List {

fn request() -> String {
 let mut text: String = String::new();

 std::io::stdin().read_line(&mut text).expect("Input failed");

 text = text.trim().to_string();
 text = text.replace("\n", "") ;
 text = text.replace("\r", "") ;

 text
}//fn request() -> String {

fn reverse(mut item: Item) -> Item {
 let mut first: Item = Item { next: None, value: item.value };

 while let Some(rest) = item.next {
  first.next = Some(Box::new(Item { next: first.next.take(), value: first.value }));

  first.value = rest.value;

  item.next = rest.next;
 }//while let Some(rest) = first.next {

 first
}//fn reverse(mut item: Item) -> Item {

fn size(mut item: Item) -> usize {
 let mut size: usize = 1;

 while let Some(rest) = item.next {
  size += 1;

  item.next = rest.next;
 }//while let Some(rest) = item.next {

 size
}//fn size(mut item: Item) -> usize {

fn swap(item: Item) -> Item {
 let mut index: usize = 0;

 let rest: usize = size(item.clone()) % 2;

 let mut reverse: Item = reverse(item);

 let mut swap: Item = Item { next: None, value: 0 };

 loop {
  index += 1;

  if index == 1 && rest == 1 {
   match reverse.pop() {
    Pop::Last(first) => {
     swap.value = first;

     break;
    }//Pop::Last(first) => {

    Pop::Next(first) => {
     swap.value = first;

    }//Pop::Next(first) => {
   }//match reverse.pop() {

  } else {//if index == 1 && rest == 1 {
   match reverse.pop() {
    Pop::Last(first) => {
     if index == 1 { swap.value = first; } else { swap.push(first); }

     break;
    }//Pop::Last(first) => {

    Pop::Next(first) => {
     match reverse.pop() {
      Pop::Last(second) => {
       if index == 1 { swap.value = second; swap.push(first); } else { swap.push(second); swap.push(first); }

       break;
      }//Pop::Last(second) => {

      Pop::Next(second) => {
       if index == 1 {
        swap.value = second;

        swap.push(first);

       } else {//if index == 1 {
        swap.push(second);

        swap.push(first);
       }//} else {//if index == 1 {
      }//Pop::Next(second) => {
     }//match reverse.pop() {
    }//Pop::Next(first) => {
   }//match reverse.pop() {
  }//} else {//if index == 1 && rest == 1 {
 }//loop {

 swap
}//fn swap(item: Item) -> Item {

fn main() {
 loop {
  println!("\r\n\r\nvector:");

  let input: String = request();

  if &input[..] == "exit" {
   break;

  } else {//if &input[..] == "exit" {
   match serde_json::from_str::<Vec<i32>>(&input[..]) {
    Ok(vector) => {
     if let Some(item) = Item::new(vector) {
      println!("\r\nswap: {:?}", Item::vector(swap(item)));

     } else {//if let Some(item) = Item::new(vector) {
      println!("\r\nswap: []");

     }//} else {//if let Some(item) = Item::new(vector) {
    }//Ok(vector) => {

    Err(error) => {
     println!("Error: {:?}", error);

    }//Err(error) => {
   }//match serde_json::from_str::<Vec<i32>>(&input[..]) {
  }//} else {//if &input[..] == "exit" {
 }//loop {
}//fn main() {
