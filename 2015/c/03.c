#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef struct {
    int x;
    int y;
} point_t;

#define EQ_POINTS(a, b) (a.x == b.x && a.y == b.y)


typedef struct {
    point_t *houses;
    size_t arr_len;
    size_t num_houses;
} house_ctx_t;


void init(house_ctx_t *ctx, point_t init_location) {

    static const size_t initial_arr_len = 256;

    ctx->houses = malloc(sizeof(point_t) * initial_arr_len);
    ctx->arr_len = 256;
    ctx->num_houses = 1;
    ctx->houses[0] = init_location;
}


void update(house_ctx_t *ctx, point_t next_location) {

    bool already_visited = false;
    for (int i=0; i<ctx->arr_len; i++) {
        if (EQ_POINTS(next_location, ctx->houses[i])) {
            already_visited = true;
        }
    }

    if (!already_visited) {
        // Double array size if out of mem
        if (ctx->num_houses == ctx->arr_len) {
            ctx->arr_len *= 2;
            ctx->houses = realloc(ctx->houses, sizeof(point_t) * (ctx->arr_len));
        }
        // Add house to array
        ctx->houses[(ctx->num_houses)++] = next_location;
    }
}


int part1(FILE *f_input) {

    rewind(f_input);
    point_t cur_location = { .x = 0, .y = 0 };
    house_ctx_t ctx;
    init(&ctx, cur_location);

    int ch = 0;
    while ( (ch = fgetc(f_input)) != EOF ) {
        switch (ch)
        {
            case '^': cur_location.y += 1; break;
            case 'v': cur_location.y -= 1; break;
            case '>': cur_location.x += 1; break;
            case '<': cur_location.x -= 1; break;
            default: break;
        }
        update(&ctx, cur_location);
    }

    return ctx.num_houses;
}


int part2(FILE *f_input) {

    rewind(f_input);
    point_t santa_location = { .x = 0, .y = 0 };
    point_t robot_location = { .x = 0, .y = 0 };
    house_ctx_t ctx;
    init(&ctx, santa_location);

    point_t *santas[2] = {&santa_location, &robot_location};
    int i = 0;

    int ch = 0;
    while ( (ch = fgetc(f_input)) != EOF ) {
        switch (ch)
        {
            case '^': santas[i]->y += 1; break;
            case 'v': santas[i]->y -= 1; break;
            case '>': santas[i]->x += 1; break;
            case '<': santas[i]->x -= 1; break;
            default: break;
        }
        update(&ctx, *santas[i]);
        i = (i + 1) % 2;
    }

    return ctx.num_houses;
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
    printf("num houses: %d\n", part1(f_input));

    printf("--- Part 2 ---\n");
    printf("num houses: %d\n", part2(f_input));

    fclose(f_input);

    return 0;
}