"use client";

import useAnchorProvider from "@/hooks/use-anchor-provider";
import TodoProgram from "@/lib/todo-program";
import { Center, Flex, List, Spinner, Text, useToast } from "@chakra-ui/react";
import { IdlAccounts } from "@coral-xyz/anchor";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { IDL } from "../../../target/types/todo_app";
import TodoItem from "./todo-item";

export default function TodoList({
  profile,
}: {
  profile: IdlAccounts<typeof IDL>["profile"];
}) {
  const provider = useAnchorProvider();
  const toast = useToast();
  const queryClient = useQueryClient();

  const { data: todos, isLoading } = useQuery({
    queryKey: ["todos", profile.key.toBase58(), profile.todoCount],
    enabled: !!profile,
    queryFn: () => new TodoProgram(provider).fetchTodos(profile),
  });

  const { mutate: toggleTodo } = useMutation({
    mutationKey: ["todos", profile.key.toBase58()],
    mutationFn: async (todoIndex: number) => {
      const program = new TodoProgram(provider);
      const tx = await program.toggleTodo(todoIndex);

      const signature = await provider.sendAndConfirm(tx);

      console.log(">> signature", signature);
      return signature;
    },
    onSuccess: (tx) => {
      console.log(tx);
      toast({
        title: "Transaction sent",
        status: "success",
      });

      return queryClient.invalidateQueries({
        queryKey: ["todos", profile.key.toBase58(), profile.todoCount],
      });
    },
    onError: (error) => {
      console.error(error);
    },
  });

  const handleToggle = (todoIndex: number) => {
    console.log("toggle");
    toggleTodo(todoIndex);
  };

  if (isLoading) {
    return (
      <Center as={Flex} direction="column" gap={4} py={8}>
        <Spinner size="xl" colorScheme="blue" />
        <Text>Loading...</Text>
      </Center>
    );
  }

  console.log("todos", todos?.length);

  console.log(
    "todo: ",
    todos?.map((t) => t.profile.toString())
  );

  return (
    <List>
      {todos?.map((todo, idx) => (
        <TodoItem
          key={idx}
          content={todo.content}
          completed={todo.completed}
          onToggle={() => handleToggle(idx)}
        />
      ))}
    </List>
  );
}
