# 8-1

```
連結リストのコード8.6において，連結リストの各ノードに格納された値を順に出力する関数printList（26～32行目）の処理に要する計算量を評価してください．
```

code8.6
https://github.com/drken1215/book_algorithm_solution/blob/master/codes/chap08/code_8_6.cpp

this function seek nodes of linked list, and print name of nodes if the next pointer is not nil.
https://github.com/drken1215/book_algorithm_solution/blob/0cda9c1f175f6f94e242ec2dd7949dcce8befcbb/codes/chap08/code_8_6.cpp#L25-L32

this procedure is performed for the number of nodes, so if we define the number of nodes, time complexity gonna be $O(N)$ .

# 8-2

```
サイズがNの連結リストにおいて，get(i)をheadからスタートしてi番目の要素を取得する関数とします．このとき以下のコードの計算量を求めてください．
```

```cpp
for (int i = 0; i < N; i++) {
    cout << get(i) << endl;
}
```

the time complexity of this function `get(N)` is $O(N)$, and
this function is called inside of the for loop, so that the time complexity of this codes are $(O(N^2)).

# 8-3

```
連結リストにおいて，サイズをO(1)で取得できるようにする方法を述べてください．
```

let each nodes have the position of the list.
if we wanna know the size of linked list, we just get the position of the end of node, and the time complexity gonna be $O(1)$.

# 8-4

```
単方向連結リストにおいて，特定のノードvを削除する方法を述べてください．ただし，O(N)の計算量を要してもよいものとします．
```

if we know the pointer of the target node, just release it after linked previous node and next node. it only takes $O(1)$ .

```cpp
void erase(Node *v) {
    if (v == nil) return; 
    v->prev->next = v->next;
    v->next->prev = v->prev;
    delete v; 
}
```

and if we don't know the pointer of target node, just seek from start like below and erase it. it takes $O(N)$ .

```cpp
for (; cur != nil; cur = cur->next) {
        // if we find target, delete it after linking previous node and next node;
}
```
