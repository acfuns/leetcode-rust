# LeetCode Solution in Rust

计划把 LeetCode 所有的题（除了 vip 题）都用 Rust 刷一遍。

主要目的练习 Rust 语法，因为里面的大多数题目我都已经做过，C++ 刷的所以这里就用 Rust 刷一遍。

目前按照难度分类，不过建议还是全局搜索题目，每个题是一个 mod, mod 是这样命名的：
```
// 因为文件不能以数字开头命令，所以加个 s 标志符
// 0001 是题号，因为 LeetCode 现在题目只有几千道，所以这里只用 4 位数字
// xxx是题目，和 LeetCode 上的题目名字保持一致, 并且所有单词小写，单词之间空格用 _ 分隔
// eg: s0001_two_sum.rs
s0001_xxx.rs
```

## Usage

```bash
$ git clone https://github.com/acfuns/leetcode-rust.git

$ cd leetcode-rust

// id is the problem id
$ cargo t test-{id}
```

## License
[MIT](https://github.com/acfuns/leetcode-rust/blob/master/LICENSE) LicenseCopyright © 2022 [acfuns](https://github.com/acfuns)
