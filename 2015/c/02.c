#include <stdio.h>
#include <stdbool.h>

#define MAX(x, y) ( x > y ? x : y )
#define MIN(x, y) ( x < y ? x : y )

void swap(int *a, int *b) { *a += *b; *b = *a - *b; *a -= *b; }

void sort3(int *a, int *b, int *c) {
    if (*a > *b) swap(a, b);
    if (*b > *c) swap(b, c);
    if (*a > *b) swap(a, b);
} 

int paper_area(int l, int w, int h) {
    return 2*l*(w + h) + 3*w*h;
}

int ribbon_len(int l, int w, int h) {
    return 2*(w + h) + l*w*h;
}

int do_box_math(char *line, int (*box_fn)(int, int, int)) {

    int a=0, b=0, c=0;
    sscanf(line, "%dx%dx%d", &a, &b, &c);

    sort3(&a, &b, &c);
    int l=c, w=b, h=a;

    return box_fn(l, w, h);
}

int sum_for_boxes(FILE *f_input, int (*box_fn)(int, int, int)) {

    int sum = 0;
    rewind(f_input);

    int ch = 0;
    char line_buf[80];
    int line_i = 0;
    while ( (ch = fgetc(f_input)) != EOF ) {
        switch (ch)
        {
            case '\r':
                continue;
                break;
            case '\n':
                line_buf[line_i] = '\0';
                sum += do_box_math(line_buf, box_fn);
                line_i = 0;
                break;
            default:
                line_buf[line_i++] = ch;
                break;
        }
    }

    return sum;
}

int part1(FILE *f_input) {
    return sum_for_boxes(f_input, &paper_area);
}

int part2(FILE *f_input) {
    return sum_for_boxes(f_input, &ribbon_len);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Error: Invalid cmd line args.");
        return 1;
    }

    FILE *f_input = fopen(argv[1], "r");
    if (f_input == NULL) {
        printf("Error: Can't read input file.");
        return 1; 
    }

    printf("--- Part 1 ---\n");
    printf("sqft paper: %d\n", part1(f_input));

    printf("--- Part 2 ---\n");
    printf("ft ribbon: %d\n", part2(f_input));

    fclose(f_input);

    return 0;
}