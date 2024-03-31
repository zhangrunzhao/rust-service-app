interface LoginProps { 
  test1: string,
}


// 该组件主要用于登录相关事项
export const Login: React.FC<LoginProps> = ({ test1}) => { 
  return <div>你好 { test1 }</div>  
}