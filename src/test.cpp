#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <poll.h>
#include <stdio.h>
#include <thread>
#include <string>
#include <cstring>
int main()
{
    int fd = open("/tmp/dlt", O_NONBLOCK | O_RDWR);
    printf("hellow \n");
    pollfd fds[1] {};
    fds[0].fd = fd;
    fds[0].events = POLLIN;
    char buf[256] {};

    //auto th = std::thread([]{
    //    for (uint8_t ii = 0; ii < 10; ++ii )
    //    {
    //        const char *a = "aaaa";
    //        int fd2 = open("/tmp/dlt", O_NONBLOCK | O_WRONLY);
    //        ssize_t size = write(fd2, a, 4);
    //        std::this_thread::sleep_for(std::chrono::seconds(1));
    //    }
    //});


    while (true)
    {
        printf("begin\n");
        poll(fds, 1, 10 * 1000);
        ssize_t size = read(fds[0].fd, buf, 256);
        printf("size = %d, event = %d, data = %s\n", size, fds[0].revents, buf);
    }
    
}