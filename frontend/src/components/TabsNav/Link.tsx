import { Link, useMatch, useResolvedPath } from 'react-router-dom';
import React from 'react';

export const TabLink = React.forwardRef<HTMLAnchorElement, any>((props, ref) => {
  const { href, as, ...rest } = props;
  let resolved = useResolvedPath(href);
  let match = useMatch({ path: resolved.pathname, end: true });

  return (
    <Link ref={ref} to={href} as={as} {...rest} className={match ? `rs-nav-item-active ${rest.className}` : rest.className} />
  );
});
