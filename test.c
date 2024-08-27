int c = 5;

int plus(int a,int b){
    return a+b;
}
int minus(int a, int b){
    return a-b;
}
int main(){
    int a = 2;
    int b = 3;
    a = a + b;
    b = minus(plus(a-b+b,b),plus(a,b));
    b;
}


