use std::time::Instant;


pub struct Task
{
    pub message: String,
    contexts: Vec<String>,
    creation_time: Instant,
}


impl Task
{
    pub fn new(message: String) -> Self
    {
        Task
        {
            message,
            contexts: Vec::<String>::new(),
            creation_time: Instant::now(),
        }
    }


    pub fn contexts(&self) -> &Vec<String> { &self.contexts }


    pub fn add_context(&mut self, context: String) -> &mut Self
    {
        self.contexts.push(context);

        self
    }
}


pub struct List
{
    pub name: String,
    tasks: Vec<Task>,
}


impl List
{
    pub fn new(name: String) -> Self
    {
        List
        {
            name,
            tasks: Vec::<Task>::new(),
        }
    }


    pub fn len(&self) -> usize
    {
        self.tasks().len()
    }


    pub fn is_empty(&self) -> bool { self.tasks.is_empty() }


    pub fn tasks(&self) -> &Vec<Task> { &self.tasks }


    pub fn mut_tasks(&mut self) -> &mut Vec<Task> { &mut self.tasks }


    pub fn push_task(&mut self, task: Task) -> &mut Self
    {
        self.tasks.push(task);

        self
    }


    pub fn remove_task(&mut self, index: usize)
    {
        self.tasks.remove(index);
    }


    pub fn move_task(&mut self, index: usize, target_list: &mut List)
    {
        target_list.tasks.push(self.tasks.remove(index));
    }
}
