import { Button } from "@arco-design/web-react"
import { useRequest } from "ahooks"
import { httpPost } from "../../utils"

interface LoginProps { 
  
}


// 该组件主要用于登录相关事项
export const Login: React.FC<LoginProps> = () => {
  
  const { run: getSome} = useRequest(async () => {
    // await httpGet("/happy?id=12345")

    await httpPost("/api/login", {
      username: "demo1",
      pwd: "welcome",
    })
  })

  
  return <>
    <div>你好, 我要登录</div> 
    <Button type="primary" onClick={getSome}>登录</Button>
  
  </> 
}