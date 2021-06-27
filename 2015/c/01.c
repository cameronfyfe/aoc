#include <stdio.h>
#include <stdbool.h>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Error: Invalid cmd line args.");
        return 1;
    }

    FILE *f_input = fopen(argv[1], "r");
    int ch = 0;
    int final_floor = 0;
    int i = 0;
    int basement_entry_position = -1;
    while ( (ch = fgetc(f_input)) != EOF ) {
        switch (ch) {
            case '(':
                final_floor++;
                break;
            case ')':
                final_floor--;
                if (final_floor < 0 && basement_entry_position == -1) {
                    basement_entry_position = i;
                }
                break;
            default:
                break;
        }
        i++;
    }

    printf("--- Part 1 ---\n");
    printf("final floor: %d\n", final_floor);

    printf("--- Part 2 ---\n");
    printf("basement entry position: %d\n", basement_entry_position);

    return 0;
}