const neonLib = require('.');

console.log(neonLib.hello()); 

const TREE_SIZE = 1_000_000;
const TEST_NUM = 10_000;
function testBTree() {
    let tree = neonLib.btreeNew();

    for (let i = 0; i < TREE_SIZE; i++) {
        let randomNum = Math.floor(Math.random() * TREE_SIZE * 10); 
        neonLib.btreeInsert(tree, randomNum, randomNum);
    }

    console.time("test time cost");
    for (let i = 0; i < TEST_NUM; i++){
        let key = Math.floor(Math.random() * TREE_SIZE);
        let result = neonLib.btreeFind(tree, key);
        if (i % 1_000 == 0) {
            console.log(`Find result for key ${key}:`, result);
        }
    }
    console.timeEnd("test time cost");
}

testBTree();