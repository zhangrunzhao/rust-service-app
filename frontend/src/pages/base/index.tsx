import {
  BrowserRouter as Router,
  Route,
  Routes,
  Navigate,
} from 'react-router-dom';
import { Empty, Home, Login, Register } from '../';

// 该组件主要用于配置路由和初始化各项服务配置
export const BaseApp = () => {
  return (
    <Router basename="/">
      <Routes>
        {/* 首页 */}
        <Route path="/" element={<Navigate to="/home" />} />
        <Route path="/home" element={<Home />} />

        {/* 账号相关页面 */}
        <Route path="/account" element={<Navigate to="/account/login" />} />
        <Route path="/account/login" element={<Login />} />
        <Route path="/account/register" element={<Register />} />

        {/* 404 页面，预期会跳转回首页 */}
        <Route path="*" element={<Empty />} />
      </Routes>
    </Router>
  );
};
