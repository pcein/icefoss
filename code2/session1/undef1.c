static void (*Do)();

static void EraseAll() {
  printf("remove all files....\n");
}

void NeverCalled() {
  Do = EraseAll;  
}

int main() {
  Do();
}
