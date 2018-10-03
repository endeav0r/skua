#include <inttypes.h>
#include <stdio.h>
#include <unistd.h>

int main () {
    setvbuf(stdin, NULL, _IONBF, 0);
    setvbuf(stdout, NULL, _IONBF, 0);

    printf("%p\n", &main);

    int running = 1;
    while (1) {
        uint8_t buf[32];

        if (read(1, buf, 1) != 1)
            break;

        switch (buf[0]) {
            case 0: {
                if (read(1, buf, 8) != 8)
                    break;
                uint8_t * ptr = *((uint8_t **) buf);
                write(0, ptr, 1);
                break;
            }
            default:
                running = 0;
                break;
        }
    }

    return 0;
}