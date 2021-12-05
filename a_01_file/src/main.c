#include <stdio.h>

int main() {
    FILE *fp;
    int bufLen = 256;
    char buf[bufLen];
    
    fp = fopen("test.txt", "r");
    fgets(buf, bufLen, fp);
    printf("%s", buf );
    fclose(fp);
}