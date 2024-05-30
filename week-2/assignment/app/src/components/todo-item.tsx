"use client";

import { Checkbox, ListItem } from "@chakra-ui/react";

export default function TodoItem({
  content,
  completed = false,
  onToggle,
}: {
  content: string;
  completed?: boolean;
  onToggle: () => void;
}) {
  return (
    <ListItem borderBottomColor="gray.500" borderBottomWidth="1px" py={4}>
      <Checkbox
        defaultChecked={completed}
        sx={{
          textDecoration: completed ? "line-through" : "initial",
        }}
        onChange={onToggle}
      >
        {content}
      </Checkbox>
    </ListItem>
  );
}
