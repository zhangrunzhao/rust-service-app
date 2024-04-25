import { Link } from 'react-router-dom';
import { useAtomValue } from 'jotai';
import { userAtom } from '@/store';

export const Home = () => {
  const userInfo = useAtomValue(userAtom);

  return (
    <div>
      首页
      {userInfo.user_id}
      <div className="w-24 flex justify-between">
        <Link to="/account/login">登录</Link>
        <Link to="/account/register">注册</Link>
      </div>
    </div>
  );
};
