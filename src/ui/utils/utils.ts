export const external = {
  invoke: (cmd: string) => {
    (window as any).external.invoke(cmd);
  },
};
