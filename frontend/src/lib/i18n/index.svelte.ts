import en from './en';
import fr from './fr';
import type { Dict, Lang } from './types';

export type { Dict, Lang };
export const dictionaries: Record<Lang, Dict> = { en, fr };

class LangState {
  current = $state<Lang>('fr');
  toggle() {
    this.current = this.current === 'fr' ? 'en' : 'fr';
  }
  get t(): Dict {
    return dictionaries[this.current];
  }
}

export const lang = new LangState();
