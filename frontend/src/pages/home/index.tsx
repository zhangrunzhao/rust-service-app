import { Link } from 'react-router-dom';

export const Home = () => {
  return (
    <div>
      首页
      <div className="w-24 flex justify-between">
        <Link to="/account/login">登录</Link>
        <Link to="/account/register">注册</Link>
      </div>
    </div>
  );
};
