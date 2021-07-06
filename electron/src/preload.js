import { contextBridge } from "electron";
import bindings from 'native-ext-loader!./../../neon/index.node'

const api = {
  startTask: () => {
    return new Promise((resolve, reject) => {
      bindings.startTask((err, res) => {
        if (err) reject(err)
        else resolve(res)
      })
    })
  },
};

contextBridge.exposeInMainWorld("api", api);
