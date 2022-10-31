import React, { useEffect } from 'react';

export default function useWindowTitle(title : string) {
  useEffect(() => {
    window.document.title = `${title} - Guilder`
  }, [title])
}
