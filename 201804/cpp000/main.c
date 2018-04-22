#include<stdio.h>

int main(){
	int n;
	char *s;

	scanf("%d",&n);
	s = n%2==0 ? "AAA" : "BBBB";
	printf("%s\n",s);

	return 0;
}
