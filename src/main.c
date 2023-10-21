#include <unistd.h>

int	main(void)
{
	while(1)
	{
		write(1, "hello world\n", 13);
	}
	return (0);
}