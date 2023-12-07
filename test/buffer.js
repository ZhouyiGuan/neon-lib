const neonLib = require('../');


const BUFFER_SIZE = 1_000_000;

function testBuffer() {
    let array = Array.from({ length:BUFFER_SIZE }, (_, index) => index);

    const before = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`uses ${Math.round(before * 100) / 100} MB`);

    console.time("buffer process cost");
    let result = neonLib.arrayDouble(array);
    console.timeEnd("buffer process cost");

    const after = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`use ${Math.round(after * 100) / 100} MB`);
}
testBuffer();

/* function testBufferJs() {
    let array = Array.from({ length:BUFFER_SIZE }, (_, index) => index);
    let result = new Array(BUFFER_SIZE);

    console.time("buffer process cost");
    for (let i = 0; i < array.length; i++) {
        result[i] = array[i] * 2;
    }
    console.timeEnd("buffer process cost");
}
testBufferJs(); */