var myName = "hello";
var myAge = 41;
var canVote = true;
var anything = "dog";
anything = 2;
document.getElementById("tsStuff").innerHTML = "My Name is " + myName;
document.write("my canVote is a " + typeof (canVote) + "<br/>");
document.write("my myName is a " + typeof (myName) + "<br/>");
document.write("my anything is a " + typeof (anything) + "<br/>");
var strToNum = parseInt("5");
var numToStr = 5;
document.write("numToStr is a " + typeof (numToStr.toString()) + "<br/>");
var PI = 3.1415926;
var superman = {
    realName: "Akira",
    superName: "Devilman"
};
document.write(superman.realName + " is " + superman.superName + "<br>");
var employees = ["Bob", "Sally", "Sam"];
employees.push("Josh");
var heros = [];
heros.push(superman);
document.write("a" + 5 + "<br>");
var sampleLet = 123;
sampleLet = 1;
document.write("sampleLet " + sampleLet + ".<br>");
var arr = [1, 2, 3, 45, 5];
// arr = arr.map(function(a){return a*2});
var getSum = function (a, b) {
    return a + b;
};
document.write("5+2=" + getSum(5, 2) + "<br>");
