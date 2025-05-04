## 编程题
- 使用git cherry-pick完成上一章的迁移
- 实现sys_spawn, 新建一个子进程直接exec对应的程序
- PCB增加stride, 在run_task时更改stride值,修改fetch得到stride最小的task
## 问答题
1. 轮不到P1执行, 因为P2的stride溢出后小于P1的stride
2. stride的最大增量为BigStride / 2, stride算法选择stride最小的, 又因为prio的值大于等于2, 因此不考虑溢出的情况下最大值和最小值的差值小于等于BigStride / 2
3. 
```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let d = self.0 - other.0;
        if abs(d * 2) > BIG_STRIDE {
            if d < 0 {Some(Ordering::Greater)} else {Some(Ordering::Less)}
        } else {
            if d < 0 {Some(Ordering::Less)} else {Some(Ordering::Greater)}
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```
## 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
    - 无
2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
    - 无
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。