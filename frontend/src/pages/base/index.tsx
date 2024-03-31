import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { Login } from '../';


// 该组件主要用于配置路由和初始化各项服务配置
export const BaseApp = () => { 
  return <Router basename="/">
      <Routes>
        <Route path="/">
          <Route path="login" Component={() => <Login test1="hello world" />} />
        </Route>
      </Routes>
  </Router>
}