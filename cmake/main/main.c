/* Hello World Example

   This example code is in the Public Domain (or CC0 licensed, at your option.)

   Unless required by applicable law or agreed to in writing, this
   software is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
   CONDITIONS OF ANY KIND, either express or implied.
*/
#include <stdio.h>

extern int rust_main(void);

void app_main(void) {
    printf("Hello world from C!\n");

    int result = rust_main();

    printf("Rust returned code: %d\n", result);
}
