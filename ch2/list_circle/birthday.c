
struct birthday
{
    int day;
    int month;
    int year;
    struct list_head_list;
};

static LIST_HEAD(birthday_list);
struct birthday *person;
person=kmalloc(sizeof(*person),GFP_KERNEL);
