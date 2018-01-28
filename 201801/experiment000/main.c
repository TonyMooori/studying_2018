#include <stdio.h>

int main(void){
    int a[5]={3,5,6,7,88};
    int b[5]={3,5,6,7,88};
    
    a[0] = printf("now");
    b[0] = printf("reading ");
    
    for(int i=0 ; i < 5 ;i++ ){
        a[i] = a[i] * 3;
        b[i] = b[i] + b[i];
    }
    
    for(int i=0 ; i < 5 ;i++ ){
        a[i] = a[i] * 3;
    }
    
    for(int i=0 ; i < 5 ;i++ ){
        a[i] = a[i] * 7;
        b[i] = b[i] + b[i] + b[i];
    }
    
    for(int i=0 ; i < 5 ;i++ ){
        a[i] = a[i] * 4;
    }
    
    printf("%d",a[0]+a[1]);
}
