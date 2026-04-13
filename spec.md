# Todo CLI — Project Spec

## Core Features

**As a user, I want to add a task with a title so that I can keep track of things I need to do.**

**As a user, I want to list all my tasks so that I can see what's pending and what's done.**

**As a user, I want to mark a task as complete by its ID so that I can track my progress.**

**As a user, I want to remove a task by its ID so that I can clean up my list.**

**As a user, I want to assign a priority (low, medium, high) to a task when I create it so that I know what's more important.**

**As a user, I want to filter my task list by priority so that I can focus on what matters most.**

---

## Input & Interaction

**As a user, I want to type commands in a loop without restarting the program so that I can manage my tasks in one session.**

**As a user, I want to type `quit` to exit the program so that I can close it cleanly.**

**As a user, I want to see a short help message listing available commands so that I know what I can do.**

---

## Error Handling & Edge Cases

**As a user, if I try to complete or remove a task with an ID that doesn't exist, I want to see a clear error message so that I understand what went wrong.**

**As a user, if I type an ID that isn't a number, I want to see an error message instead of the program crashing.**

**As a user, if I try to complete a task that's already complete, I want to be warned so that I know it was already done.**

**As a user, if I submit an empty command or just press enter, I want the program to ask me again instead of crashing.**

**As a user, if I add a task with an empty title, I want to see an error telling me the title can't be blank.**

---

## Display

**As a user, I want each task displayed with its ID, completion status, priority, and title so that I can see all relevant info at a glance.**

**As a user, I want completed tasks to look visually distinct from pending ones so that I can tell them apart quickly.**
