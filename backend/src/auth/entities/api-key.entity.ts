import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
  UpdateDateColumn,
  ManyToOne,
  JoinColumn,
  Index,
} from 'typeorm';
import { User } from '../../users/entities/user.entity';

@Entity('api_keys')
export class ApiKey {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Index()
  @Column({ type: 'uuid' })
  userId: string;

  @ManyToOne(() => User, { onDelete: 'CASCADE' })
  @JoinColumn({ name: 'userId' })
  user: User;

  /** Human-readable label set by the owner */
  @Column({ type: 'varchar', length: 100 })
  name: string;

  /** First 8 chars of the raw key — safe to display, not secret */
  @Index()
  @Column({ name: 'key_prefix', type: 'varchar', length: 12 })
  key_prefix: string;

  /** bcrypt hash of the full raw key — never exposed via API */
  @Column({ name: 'key_hash', type: 'varchar' })
  key_hash: string;

  /**
   * Scopes this key is authorised to access.
   * Stored as a Postgres text[] column.
   * Example values: 'predictions:read', 'markets:read', 'webhooks:write'
   */
  @Column({ type: 'text', array: true, default: () => "'{}'::text[]" })
  scopes: string[];

  /** Optional expiry date — null means the key never expires */
  @Column({ name: 'expires_at', type: 'timestamptz', nullable: true })
  expires_at: Date | null;

  /**
   * Set when the owner explicitly revokes the key.
   * A non-null value means the key is revoked.
   */
  @Column({ name: 'revoked_at', type: 'timestamptz', nullable: true })
  revoked_at: Date | null;

  /**
   * Throttled write — updated at most once every 60 s per guard request.
   * Avoids a DB write on every single authenticated request.
   */
  @Column({ name: 'last_used_at', type: 'timestamptz', nullable: true })
  last_used_at: Date | null;

  @CreateDateColumn({ type: 'timestamptz' })
  created_at: Date;

  @UpdateDateColumn({ type: 'timestamptz' })
  updated_at: Date;

  /** Convenience getter: true when the key is currently usable */
  get isActive(): boolean {
    if (this.revoked_at) return false;
    if (this.expires_at && this.expires_at < new Date()) return false;
    return true;
  }
}
