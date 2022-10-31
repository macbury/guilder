import React, { useRef, forwardRef, useEffect } from 'react';
import FunnelIcon from '@rsuite/icons/Funnel';
import Whisper from 'rsuite/Whisper';
import Popover from 'rsuite/Popover';
import IconButton from 'rsuite/IconButton';
import styled from 'styled-components';

const PopoverContainer = styled.div`
  width: 300px;
`;

export interface IFilterProps {
  children: any
}

const MenuPopover = forwardRef<any>(({ children, ...rest }, ref) => {
  return (
    <Popover ref={ref} {...rest} full>
      <PopoverContainer>
        {children}
      </PopoverContainer>
    </Popover>
  )
});

export function FilterPopup({ children } : IFilterProps) {
  const ref = useRef<any>();

  return (
    <Whisper
      preventOverflow
      placement="bottomStart"
      controlId="filter"
      trigger="click"
      ref={ref}
      speaker={<MenuPopover>{children}</MenuPopover>}>
      <IconButton icon={<FunnelIcon />} size="lg" />
    </Whisper>
  )
}
