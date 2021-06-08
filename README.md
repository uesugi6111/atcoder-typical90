# typical90
管理がめんどくさいのでリポジトリを切る



```
cargo compete new typical90 &&
mv ./typical90/Cargo.toml ./atcoder-typical90/Cargo.toml &&
rm -r ./typical90 &&
cargo compete r testcases --manifest-path ./atcoder-typical90/Cargo.toml --overwrite 
```
