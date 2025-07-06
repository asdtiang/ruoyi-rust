#[tokio::test]
async fn test_utf_8()  {
    let s = "你好";
    println!("字符数: {}", s.chars().count()); // 输出: 2
    println!("字节数: {}", s.len()); // 输出: 6
}